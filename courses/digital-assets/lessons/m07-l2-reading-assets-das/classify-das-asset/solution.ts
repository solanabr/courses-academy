// classify-das-asset: reference solution

interface DasAssetDetails {
  compression?: { compressed?: boolean };
  token_info?: { price_info?: { price_per_token?: number } };
}

interface AssetClassification {
  category: 'compressed-nft' | 'nft' | 'fungible' | 'other';
  compressed: boolean;
  fungible: boolean;
  pricePerToken: number | null;
  requiresDasRpc: boolean;
}

function classifyAsset(iface: string, detailsJson: string): AssetClassification {
  const details = JSON.parse(detailsJson) as DasAssetDetails;

  const NFT_INTERFACES = new Set<string>([
    'V1_NFT',
    'V1_PRINT',
    'V2_NFT',
    'LEGACY_NFT',
    'ProgrammableNFT',
    'MplCoreAsset',
    'MplBubblegumV2',
  ]);
  const FUNGIBLE_INTERFACES = new Set<string>(['FungibleAsset', 'FungibleToken']);

  const compressed = details.compression?.compressed === true;
  const fungible = FUNGIBLE_INTERFACES.has(iface);
  const isNft = NFT_INTERFACES.has(iface);
  const pricePerToken = details.token_info?.price_info?.price_per_token ?? null;

  let category: AssetClassification['category'] = 'other';
  if (fungible) {
    category = 'fungible';
  } else if (isNft) {
    category = compressed ? 'compressed-nft' : 'nft';
  }

  return {
    category,
    compressed,
    fungible,
    pricePerToken,
    requiresDasRpc: compressed,
  };
}
