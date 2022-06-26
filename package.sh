trunk build; 
rm -r index_folder/ ; cargo run --bin generate resources/resources.csv index_folder; cp -r index_folder/  dist/;
cp -r resources/ dist/