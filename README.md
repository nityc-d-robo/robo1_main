git clone https://github.com/nityc-d-robo/robo1_main &&  
cd  robo1_main &&  
chmod +x ./start.sh &&  
cp ./start.sh ~/ &&  
(crontab -l 2>/dev/null; echo "@reboot /home/$USER/start.sh") | crontab -
