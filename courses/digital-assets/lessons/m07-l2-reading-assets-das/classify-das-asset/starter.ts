// classify-das-asset: the core of read-any-asset.ts
//
// One DAS getAsset response can describe a legacy NFT, a Metaplex Core asset,
// a Bubblegum v2 compressed NFT, or a fungible token. Your reader has to route
// each one correctly WITHOUT extra RPC calls, using only the fields DAS returns.
//
// Calling convention: the grader invokes classifyAsset(iface, detailsJson):
//   - iface:       the DAS `interface` string, e.g. 'MplBubblegumV2'
//   - detailsJson: the REST of the getAsset response as a JSON string;
//                  JSON.parse it to reach compression / token_info
//
// Return an AssetClassification:
//   - category:       'compressed-nft' | 'nft' | 'fungible' | 'other'
//   - compressed:     true only when compression.compressed === true
//   - fungible:       true for the fungible interfaces
//   - pricePerToken:  token_info.price_info.price_per_token, or null if absent
//   - requiresDasRpc: true when the asset is compressed (a cNFT has no account of
//                     its own: only a DAS RPC can reconstruct it)
//
// Interface enum reference (DAS spec 1.1.0):
//   NFT-ish : V1_NFT, V1_PRINT, V2_NFT, LEGACY_NFT, ProgrammableNFT,
//             MplCoreAsset, MplBubblegumV2
//   Fungible: FungibleAsset, FungibleToken

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
  // TODO: branch on iface and details.compression?.compressed
  // TODO: pull the price from details.token_info?.price_info?.price_per_token
  return {
    category: 'other',
    compressed: false,
    fungible: false,
    pricePerToken: null,
    requiresDasRpc: false,
  };
}
