# Paketbote

This service can act as your personal Arch Linux package repository. After creating an account, simply upload your packages with curl (or any other HTTP client of your choice):

```
export TOKEN="[api token]"
export PKG="linux-mainline-5.7rc3-1-x86_64.pkg.tar.xz"

curl http://upload.paketbote.tk/username/repository \
	-H "Authorization: Bearer $TOKEN" \
	-F "package=@$PKG" \
	-F "signature=@$PKG.sig"
```

Configure your personal repository in /etc/pacman.conf:

```
[username]
Server = https://paketbote.tk/username/repository
```

And install packages with pacman:

```
sudo pacman -Syu linux-mainline
```
