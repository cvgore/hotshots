<?php

// https://mobileapi.x-kom.pl/api/v1/xkom/hotShots/current?onlyHeader=false&commentAmount=15

$headers = array_filter($_SERVER, function (string $key) {
    return mb_stripos($key, 'HTTP_') === 0;
}, ARRAY_FILTER_USE_KEY);

foreach ($headers as $name => $value) {
    $oldname = $name;
    $name = mb_substr($name, 5);
    $name = str_replace('_', '-', trim($name));

    $headers[$name] = $value;
    unset($headers[$oldname]);
}

file_put_contents('php://stderr', json_encode([
    'headers' => $headers
]));

match ($_GET['hotshot'] ?? false) {
    'xkom' => xkom($headers),
    default => notfound($headers),
};

function notfound($headers) {
    http_response_code(404);
    echo 'not found';
}

function xkom($headers) {
    foreach ($headers as $name => $value) {
        if (mb_stripos($name, 'X-API-Key') === 0
            && mb_strpos($value, 'apikey') === 0) {
            goto allowed;
        }
    }

    http_response_code(403);
    echo 'unauthorized';
    return;

allowed:

    header('Content-Type: application/json');
    readfile('xkom.json');
}