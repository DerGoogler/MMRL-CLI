SKIPMOUNT=false
PROPFILE=false
POSTFSDATA=false
LATESTARTSERVICE=false

print_modname() {
    ui_print "    __  _____  _______  __ "
    ui_print "   /  |/  /  |/  / __ \/ / "
    ui_print "  / /|_/ / /|_/ / /_/ / /  "
    ui_print " / /  / / /  / / _, _/ /___"
    ui_print "/_/  /_/_/  /_/_/ |_/_____/"
}

on_install() {
    ui_print "- Extracting module files"
    unzip -qq -o "$ZIPFILE" 'system/*' -d $MODPATH >&2
    [ -d "$MODPATH/system/bin/" ] || mkdir -p "$MODPATH/system/bin/"
    ln -sf mmrl $MODPATH/system/bin/mrepo
    ln -sf mmrl $MODPATH/system/bin/foxmmm
    ln -sf mmrl $MODPATH/system/bin/amm

}

set_permissions() {
    # The following is the default rule, DO NOT remove
    set_perm_recursive $MODPATH 0 0 0755 0644
    set_perm $MODPATH/system/bin/mmrl 0 0 0755
    set_perm $MODPATH/system/bin/mrepo 0 0 0755
    set_perm $MODPATH/system/bin/foxmmm 0 0 0755
    set_perm $MODPATH/system/bin/amm 0 0 0755
}