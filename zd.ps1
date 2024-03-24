if ($args[0] -eq "-l") {
    $values = izi_directory_finder.exe @args
    foreach ($value in $values) {
        Write-Output $value
    }

    return;
}

$result = izi_directory_finder.exe @args
if ($result -ne "") {
    Set-Location $result
}
else {
    Write-Output "No matches"
}