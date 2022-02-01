@echo -off
if exist fs0:\efi\boot\main.efi then
    .\efi\boot\main.efi
else
    echo "error loading uefi app"
endif