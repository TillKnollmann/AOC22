param(
    [Parameter()]
    [Int]$day
)

$year = 2022

cargo scaffold $day
cargo download $day -y $year