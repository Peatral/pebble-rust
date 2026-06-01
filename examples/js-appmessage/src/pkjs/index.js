Pebble.addEventListener('ready', function () {
    console.log('PKJS Ready!')
    var dict = {
        'App_ExampleKey': 'Hello from JavaScript!'
    }
    Pebble.sendAppMessage(dict)
})