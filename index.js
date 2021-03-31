window.onload = function () {
    let DT = 0.05;
    setInterval(function() {
        let ticker = document.getElementById("ticker");
        if (ticker !== null) {
            ticker.click();
        }
    }, DT * 1000);
}
