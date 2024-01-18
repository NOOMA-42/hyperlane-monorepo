import { Deployer } from '@matterlabs/hardhat-zksync-deploy';
import '@matterlabs/hardhat-zksync-node/dist/type-extensions';
import '@matterlabs/hardhat-zksync-verify/dist/src/type-extensions';
import dotenv from 'dotenv';
import { ethers } from 'ethers';
import * as hre from 'hardhat';
import { Provider, Wallet } from 'zksync-ethers';

import { DeployContractOptions } from './types';

export abstract class HyperlaneDeployer {
  async deployContract(
    contractArtifactName: string,
    constructorArguments?: any[],
    options?: DeployContractOptions,
  ) {
    const log = (message: string) => {
      if (!options?.silent) console.log(message);
    };

    log(`\nStarting deployment process of "${contractArtifactName}"...`);

    const wallet = options?.wallet ?? getWallet();
    const deployer = new Deployer(hre, wallet);
    const artifact = await deployer
      .loadArtifact(contractArtifactName)
      .catch((error) => {
        if (
          error?.message?.includes(
            `Artifact for contract "${contractArtifactName}" not found.`,
          )
        ) {
          console.error(error.message);
          throw `⛔️ Please make sure you have compiled your contracts or specified the correct contract name!`;
        } else {
          throw error;
        }
      });

    // Estimate contract deployment fee
    const deploymentFee = await deployer.estimateDeployFee(
      artifact,
      constructorArguments || [],
    );
    log(`Estimated deployment cost: ${ethers.formatEther(deploymentFee)} ETH`);

    // Check if the wallet has enough balance
    await verifyEnoughBalance(wallet, deploymentFee);

    // Deploy the contract to zkSync
    const contract = await deployer.deploy(artifact, constructorArguments);
    const address = await contract.getAddress();
    const constructorArgs =
      contract.interface.encodeDeploy(constructorArguments);
    const fullContractSource = `${artifact.sourceName}:${artifact.contractName}`;

    // Display contract deployment info
    log(`\n"${artifact.contractName}" was successfully deployed:`);
    log(` - Contract address: ${address}`);
    log(` - Contract source: ${fullContractSource}`);
    log(` - Encoded constructor arguments: ${constructorArgs}\n`);

    if (!options?.noVerify && hre.network.config.verifyURL) {
      log(`Requesting contract verification...`);
      await verifyContract({
        address,
        contract: fullContractSource,
        constructorArguments: constructorArgs,
        bytecode: artifact.bytecode,
      });
    }

    return contract;
  }
}
