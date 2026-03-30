if [ ! -d /usr/lib/ ]
then
    ./nix/setup.sh
else
    sudo mkdir -p /usr/lib/epiclang/
    sudo cp -r ./plugins /usr/lib/epiclang/
    make install
    chmod +x epiclang
    sudo cp epiclang /usr/bin/epiclang
    echo "Installation complete. Plugins have been copied to /usr/lib/epiclang/plugins/"
fi