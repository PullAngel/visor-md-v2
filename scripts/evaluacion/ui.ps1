param(
  [Parameter(Mandatory=$true)][string]$Name,
  [Parameter(Mandatory=$true)][string]$Cmd,
  [string]$Out, [int]$X=0, [int]$Y=0, [int]$X2=0, [int]$Y2=0, [int]$Delta=0,
  [string]$Key, [string]$Text, [string]$File, [int]$Width=0, [int]$Height=0
)
# Herramienta de prueba: maneja UNA copia privada de la app por mensajes de
# ventana. No mueve el mouse real ni usa el portapapeles del usuario.
$ErrorActionPreference = "Stop"
Add-Type -AssemblyName System.Drawing
if (-not ("U" -as [type])) {
Add-Type @"
using System; using System.Runtime.InteropServices;
public class U {
 [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h, IntPtr hdc, uint f);
 [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
 [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint m, IntPtr w, IntPtr l);
 [DllImport("user32.dll")] public static extern uint MapVirtualKey(uint c, uint t);
 [DllImport("user32.dll")] public static extern short VkKeyScan(char c);
 [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
 [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr a, int x, int y, int cx, int cy, uint f);
 public struct RECT { public int L,T,R,B; } }
"@ }
[U]::SetProcessDPIAware() | Out-Null
$Base = Split-Path -Parent $PSCommandPath
$Home_ = Join-Path $Base $Name
$Exe = Join-Path $Home_ "$Name.exe"

function Win {
  $p = Get-Process $Name -EA SilentlyContinue | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
  if (-not $p) { throw "la app '$Name' no está abierta; usa -Cmd launch" }
  return $p.MainWindowHandle
}
function Focus($h){ [U]::PostMessage($h,0x0007,[IntPtr]0,[IntPtr]0) | Out-Null; Start-Sleep -m 40 }
function MkPoint($x,$y){ [IntPtr](($y -shl 16) -bor ($x -band 0xFFFF)) }
$VK = @{ esc=0x1B; enter=0x0D; tab=0x09; back=0x08; del=0x2E; up=0x26; down=0x28; left=0x25; right=0x27;
  home=0x24; end=0x23; pgup=0x21; pgdn=0x22; space=0x20; f1=0x70; f2=0x71; f3=0x72; f4=0x73; f5=0x74; f6=0x75;
  f7=0x76; f8=0x77; f9=0x78; f10=0x79; f11=0x7A; f12=0x7B; ctrl=0x11; shift=0x10; alt=0x12 }
$EXT = @(0x26,0x28,0x25,0x27,0x24,0x23,0x21,0x22,0x2E)
function KeyMsg($h,$vk,$down){
  $sc = [U]::MapVirtualKey($vk,0); $l = 1 -bor ($sc -shl 16)
  if ($EXT -contains $vk) { $l = $l -bor (1 -shl 24) }
  if (-not $down) { $l = [long]$l -bor 0xC0000000 }
  [U]::PostMessage($h, $(if($down){0x100}else{0x101}), [IntPtr]$vk, [IntPtr][long]$l) | Out-Null
}
function Shot($h,$path){
  $r = New-Object U+RECT; [U]::GetClientRect($h,[ref]$r) | Out-Null
  $bmp = New-Object System.Drawing.Bitmap $r.R,$r.B; $g=[System.Drawing.Graphics]::FromImage($bmp)
  $dc=$g.GetHdc(); [U]::PrintWindow($h,$dc,3)|Out-Null; $g.ReleaseHdc($dc); $bmp.Save($path); $g.Dispose(); $bmp.Dispose()
  "captura $($r.R)x$($r.B) -> $path"
}

switch ($Cmd) {
 "launch" {
   Get-Process $Name -EA SilentlyContinue | Stop-Process -Force
   $profile = Join-Path $Home_ "perfil"; New-Item -ItemType Directory -Force $profile | Out-Null
   $psi = New-Object System.Diagnostics.ProcessStartInfo $Exe
   if ($File) { $psi.Arguments = "`"$File`"" }
   $psi.UseShellExecute = $false
   $psi.EnvironmentVariables["LOCALAPPDATA"] = $profile
   $psi.EnvironmentVariables["VISOR_EVAL"] = "1"
   [System.Diagnostics.Process]::Start($psi) | Out-Null
   Start-Sleep 3; $h = Win
   if ($Width -gt 0) { [U]::SetWindowPos($h,[IntPtr]0,40,40,$Width,$Height,0x14) | Out-Null; Start-Sleep -m 500 }
   "abierta"
 }
 "close" { Get-Process $Name -EA SilentlyContinue | Stop-Process -Force; "cerrada" }
 "resize" { $h=Win; [U]::SetWindowPos($h,[IntPtr]0,40,40,$Width,$Height,0x14) | Out-Null; Start-Sleep -m 500; "tamano $Width x $Height" }
 "shot" { Shot (Win) $Out }
 "move" { $h=Win; [U]::PostMessage($h,0x200,[IntPtr]0,(MkPoint $X $Y))|Out-Null; Start-Sleep -m 250; "ok" }
 "click" { $h=Win; $p=(MkPoint $X $Y); foreach($m in 0x200,0x201,0x200,0x202){ [U]::PostMessage($h,$m,[IntPtr]$(if($m -eq 0x201){1}else{0}),$p)|Out-Null }; Start-Sleep -m 350; "ok" }
 "dblclick" { $h=Win; $p=(MkPoint $X $Y); foreach($m in 0x200,0x201,0x202,0x200,0x203,0x202){ [U]::PostMessage($h,$m,[IntPtr]$(if($m -eq 0x201 -or $m -eq 0x203){1}else{0}),$p)|Out-Null }; Start-Sleep -m 350; "ok" }
 "rclick" { $h=Win; $p=(MkPoint $X $Y); foreach($m in 0x200,0x204,0x200,0x205){ [U]::PostMessage($h,$m,[IntPtr]$(if($m -eq 0x204){2}else{0}),$p)|Out-Null }; Start-Sleep -m 350; "ok" }
 "drag" { $h=Win; [U]::PostMessage($h,0x200,[IntPtr]0,(MkPoint $X $Y))|Out-Null; [U]::PostMessage($h,0x201,[IntPtr]1,(MkPoint $X $Y))|Out-Null
   for($i=1;$i -le 8;$i++){ $ix=[int]($X+($X2-$X)*$i/8); $iy=[int]($Y+($Y2-$Y)*$i/8); [U]::PostMessage($h,0x200,[IntPtr]1,(MkPoint $ix $iy))|Out-Null }
   [U]::PostMessage($h,0x202,[IntPtr]0,(MkPoint $X2 $Y2))|Out-Null; Start-Sleep -m 350; "ok" }
 "wheel" { $h=Win; $wr=New-Object U+RECT; [U]::GetWindowRect($h,[ref]$wr)|Out-Null
   $wp=[IntPtr]([int]($Delta -shl 16)); [U]::PostMessage($h,0x200,[IntPtr]0,(MkPoint $X $Y))|Out-Null; [U]::PostMessage($h,0x20A,$wp,(MkPoint ($wr.L+$X) ($wr.T+$Y)))|Out-Null; Start-Sleep -m 300; "ok" }
 "key" {
   Focus (Win)
   # -Key acepta combinaciones: "ctrl+s", "shift+tab", "f2", "esc", "a".
   $h=Win; $parts = $Key.ToLower().Split('+'); $mods=@(); $main=$parts[-1]
   foreach($m in $parts[0..($parts.Length-2)]){ if($parts.Length -gt 1){ $mods += $VK[$m] } }
   $vk = if ($VK.ContainsKey($main)) { $VK[$main] } else { [int][char]$main.ToUpper() }
   foreach($m in $mods){ KeyMsg $h $m $true; Start-Sleep -m 30 }
   KeyMsg $h $vk $true; Start-Sleep -m 50; KeyMsg $h $vk $false
   foreach($m in $mods){ Start-Sleep -m 30; KeyMsg $h $m $false }
   Start-Sleep -m 350; "ok"
 }
 "type" {
   $h=Win; Focus $h; Start-Sleep -m 200
   foreach($c in $Text.ToCharArray()){
     $s=[U]::VkKeyScan($c); $vk=$s -band 0xFF
     if ($c -eq "`n") { KeyMsg $h 0x0D $true; KeyMsg $h 0x0D $false; Start-Sleep -m 40; continue }
     if ($vk -ne 0xFF) { KeyMsg $h $vk $true }
     [U]::PostMessage($h,0x102,[IntPtr][int]$c,[IntPtr]1)|Out-Null
     if ($vk -ne 0xFF) { KeyMsg $h $vk $false }
     Start-Sleep -m 35
   }
   Start-Sleep -m 300; "ok"
 }
 "answer" {
   # Prepara la respuesta del PRÓXIMO diálogo de archivo (Abrir, Guardar,
   # Guardar como, Exportar PDF, Elegir carpeta). En esta build de evaluación
   # el diálogo no se ve: la app toma esta respuesta. -Text "" = Cancelar.
   $f = Join-Path (Join-Path $Home_ "perfil") "eval-dialogo.txt"
   [IO.File]::WriteAllText($f, $Text, (New-Object System.Text.UTF8Encoding $false)); "respuesta lista: '$Text'"
 }
 default { throw "comando desconocido: $Cmd" }
}
