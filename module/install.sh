SKIPMOUNT=false
PROPFILE=false
POSTFSDATA=false
LATESTARTSERVICE=false

REPLACE=""


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
}

set_permissions() {
    # The following is the default rule, DO NOT remove
    set_perm_recursive $MODPATH 0 0 0755 0644
    set_perm $MODPATH/system/bin/mmrl 0 0 0755
}