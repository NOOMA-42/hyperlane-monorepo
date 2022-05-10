export const addresses = {
  alfajores: {
    upgradeBeaconController: '0x1f15e8b9C5ABf2D6f7f9033f95Fcd66b90c8aa27',
    abacusConnectionManager: '0xD57Ff4d8d3872322bD628ed8f9C6bB3617A5ef13',
    interchainGasPaymaster: '0x63BDfEF81688ddd5f135D8a8680f942205E030c2',
    inboxes: {
      kovan: {
        proxy: '0x0fda1FDfEaDADdAE9C07C2bdA184935E317688C6',
        implementation: '0x9632B89f335F6f48d8ea32fC95ccc2eA36B33570',
        beacon: '0xc8571652Fe648d2873Af278C4982Ecd07dBb0870',
        validatorManager: '0x78BEAd8669B1657BCA3F7aC202D8098f37Eb6c6A',
      },
    },
    outbox: {
      validatorManager: '0xCfC4862Df4544923265873f2d288AcEd9919d679',
      proxy: '0x71f37188d27788ea86E12557edbc9b07E5BDdf03',
      implementation: '0x3219827B9D028Cec62b996147076b4cFCeac282a',
      beacon: '0xe0B98662f15B12E4aCfd950cBe53FAad4b0C0b98',
    },
  },
  kovan: {
    upgradeBeaconController: '0x74A77C554685651F7dB3784359E1436604794438',
    abacusConnectionManager: '0x3c3b0367A960a6ea1cDe58Bb8A7E33bfC38554D3',
    interchainGasPaymaster: '0x33bE4eAa1A7c04E6Bba63e4F9cfff1eb98CF59F0',
    outbox: {
      validatorManager: '0x3f0e80FfACE85DC573F57b2f552f6F98Fc5984c4',
      proxy: '0x24D05ae06F3Ce4CF146958714c2B2FBE8B6a29c4',
      implementation: '0xbA2C920a863122a42c249d89D5954506F00513C3',
      beacon: '0xE8bF352efbC50754442095a04e81A5141fe75e16',
    },
    inboxes: {
      alfajores: {
        validatorManager: '0x63C950170534907d40B77ecF61bd07980B51f2a4',
        proxy: '0x125379056774C4246d016b2C58c6fbb80ab8829b',
        implementation: '0x6D91F26Fb1430bC0F557cC8BD7F815b441541c68',
        beacon: '0xdCCA08dC4ec34d58f69415b0C7C8e0042779c559',
      },
    },
  },
};