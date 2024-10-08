// These chains may be any protocol type.
// Placing them here instead of adjacent chains file to avoid circular dep
export const mainnet3SupportedChainNames = [
  'ancient8',
  'alephzeroevm',
  'arbitrum',
  'astar',
  'astarzkevm',
  'avalanche',
  'base',
  'bitlayer',
  'blast',
  'bob',
  'bsc',
  'celo',
  'cheesechain',
  'chiliz',
  'coredao',
  'cyber',
  'degenchain',
  'dogechain',
  'eclipsemainnet',
  'endurance',
  'ethereum',
  'everclear',
  'flare',
  'fraxtal',
  'fusemainnet',
  'gnosis',
  'immutablezkevm',
  'inevm',
  'injective',
  'kroma',
  'linea',
  'lisk',
  'lukso',
  'lumia',
  'mantapacific',
  'mantle',
  'merlin',
  'metis',
  'mint',
  'mode',
  'molten',
  'moonbeam',
  'neutron',
  'oortmainnet',
  'optimism',
  'osmosis',
  'polygon',
  'polygonzkevm',
  'proofofplay',
  'rari',
  'real',
  'redstone',
  'rootstock',
  'sanko',
  'scroll',
  'sei',
  'shibarium',
  'solanamainnet',
  'superposition',
  'taiko',
  'tangle',
  'viction',
  'worldchain',
  'xai',
  'xlayer',
  'zetachain',
  'zircuit',
  'zoramainnet',
] as const;

export const supportedChainNames = [...mainnet3SupportedChainNames];
