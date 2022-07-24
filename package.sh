rm -r ~/scratch/NTC_resource_library
mkdir -p ~/scratch/NTC_resource_library
mkdir -p index_folder/; rm -r index_folder/ ; cargo run --example generate resources/resources.csv index_folder; cp -r index_folder/  ~/scratch/NTC_resource_library;
cp -r resources/ ~/scratch/NTC_resource_library
trunk build; 
cp -r dist/ ~/scratch/NTC_resource_library;