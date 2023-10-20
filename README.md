# Magisk Module Repo Loader CLI

MMRL comes now as command line interface, with multi module install support!

## Repositoreis

You can use any repo you want, the only requirement it that the repo supports the [MRepo-Format](https://github.com/ya0211/magisk-modules-repo-util)

You can load another repo with
```shell
mmrl --repo "https://apt.izzysoft.de/magisk/json/modules.json" <OPT> <COMMAND>
```

Install a module with it

```shell
# Install aliases are "add" and "get"
mmrl --repo "https://apt.izzysoft.de/magisk/json/modules.json" install app-data-file-exec
```

Or just create a binary, `/system/bin/mmrl-izzy`

```shell
#!/system/bin/sh
mmrl --repo "https://apt.izzysoft.de/magisk/json/modules.json" @$
```

then
```shell
mmrl-izzy install app-data-file-exec data_isolation_support # supports multi module install
```

## Get informations

The MMRL CLI also supports getting infos of the module

just run
```shell
mmrl info mkshrc

# or
mmrl-izzy info app-data-file-exec
```


## Searching

Wanna search some module? You can do it.

```shell
mmrl search id node # mmrl lookup all "hide user"

# or 
mmrl-izzy search all aosp
```

Get all filter

```shell
mmrl search 

# or
mmrl-izzy search
```

## Downloading

Downloading just the module is also posible

```shell
mmrl download mkshrc node_on_android # --version 152
```

## Installing modules

Maybe simple...

```shell
mmrl install mkshrc@1.3.4 node_on_android
```

## Install Viper4Android

> Requires version `0.1.1`

```shell
mmrl install -y "https://zackptg5.com/downloads/Audio-Modification-Library_v5.1.zip" "https://john-fawkes.com/Downloads/acp.zip" "https://github.com/AndroidAudioMods/ViPER4Android/releases/download/v0.5.0/V4A_Magisk_Module_0.5.0.zip"
```