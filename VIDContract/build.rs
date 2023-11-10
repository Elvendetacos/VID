
use io::NftMetadata;
use gear_wasm_builder::WasmBuilder;
use gmeta::Metadata;


fn main(){
   WasmBuilder::with_meta(NftMetadata::repr()).exclude_features(vec!["binary-vendor"]).build();
}