Pebble.addEventListener('ready', function () {
    console.log('PKJS Ready!')
    var dict = {
        'EXAMPLE': 'Hello from JavaScript!'
    }
    Pebble.sendAppMessage(dict)
})