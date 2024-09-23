git clone https://github.com/nityc-d-robo/robo1_main &&  
cd  robo1_main &&  
mv ./start.sh ~/ &&  
sudo echo "@reboot /home/$USER/start.sh" >> /tmp/crontab.3D2WjX/crontab 
