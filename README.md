# What is this?
A script which reposts one or more posts to a Lemmy community from a selected subreddit. In construction currently.

Only posts that have atleast 200 upvotes can be reposted and whenever a post is made, a "reddit_to_lemmy_reposter" folder will be created in the user's Documents folder to store already posted posts's ids in a textfile to avoid duplicates.

This guide was made for **Linux** and haven't been tested on Windows or MacOS.

# Build and setup
First you have to clone the repository, and build a release version of the script
```
git clone https://github.com/NoobKozlegeny/reddit_to_lemmy_reposter
cd reddit_to_lemmy_reposter
cargo build --release
```

Secondly you should create a bot account on Lemmy on your preferred instance. Don't forget to **set the account as bot in settings**.

Thirdly subscribe to your preferred community where the bot will repost to.

From now on you have 2 options to choose from, if you want to run it.

1. Run it manually
2. Run it as a systemd service

## Run it manually
Before initiating the script you need to set the bot's registered name and password. (Name and password you registered on Lemmy)
```
export NAME_OR_EMAIL=<INSERT_NAME>
export LEMMY_AUTH_PASSWORD=<INSERT_PASSWORD>
```

It's important to note that the **Reddit subreddit's name and Lemmy community's name needs to be the SAME**
```
reddit_to_lemmy_reposter <INSTANCE_WHERE_THE_BOT_WAS_CREATED> <SUBREDDIT_TO_REPOST_FROM>
```

### Example
If you have a bot which was created on the lemmy.basedcount.com instance and should repost to c/2visegrad4you then you need to run this command:
```
reddit_to_lemmy_reposter lemmy.basedcount.com 2visegrad4you
```

## Run it as a systemd service
This choice is preferable if you wish to execute the script periodically. You need to create a .service file and a .timer file in /etc/systemd/system for this option.

Firstly you need to create the .service file with the relevant informations. You need to add the bot's registered name and password into the .service file. (Name and password you registered on Lemmy)

### Example
If you have a bot which was created on the lemmy.basedcount.com instance and should repost to c/2visegrad4you then you need to run this command:

1. <INSERT_BOT_NAME>: Bot's name on Lemmy
2. <INSERT_BOT_PASSWORD>: Bot's password on Lemmy
3. <INSERT_LINUX_USER>: Linux user's name. You can get this via ```echo $USER```
4. <INSERT_ABSOLUTE_PATH_TO_SCRIPT>: Absolute path to the script's executable (Ex: /home/privatenoob/Codes/reddit_to_lemmy_reposter/target/release/reddit_to_lemmy_reposter)

```
[Unit]
Description=Repost 1 Reddit content periodically to c/2visegrad4you from r/2visegrad4you

[Service]
Type=simple
Environment="NAME_OR_EMAIL=<INSERT_BOT_NAME>"
Environment="LEMMY_AUTH_PASSWORD=<INSERT_BOT_PASSWORD>"
User=<INSERT_LINUX_USER>
ExecStart=<INSERT_ABSOLUTE_PATH_TO_SCRIPT> $OPTIONS lemmy.basedcount.com 2visegrad4you
```

Secondly you need a .timer file which name has to be equal to the .service one.

1. OnBootSec: When to run the script after a boot have happened.
2. OnUnitActiveSec: Sets how often the .service file should be run.

```
[Unit]
Description=Timer for reddit-to-lemmy-reposter to repost from r/2visegrad4you

[Timer]
OnBootSec=1min
OnUnitActiveSec=4h

[Install]
WantedBy=timers.target
```