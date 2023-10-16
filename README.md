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
mmrl search node # mmrl lookup "hide user"

# or 
mmrl-izzy search aosp
```

Display all modules

```shell
mmrl search all

# or
mmrl-izzy search all
```

## Downloading

Downloading just the module is also posible

```shell
mmrl download mkshrc node_on_android
```

## Installing modules

Maybe simple...

```shell
mmrl install mkshrc node_on_android
```