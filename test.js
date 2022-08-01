let maximumResponseTime = 200;
let iterations = 100;
let delay = 100;
let responseTimes = [];
let i=0;

function sendRequest() {
    pm.sendRequest({
        url: pm.request.url.toString(),
        method: 'GET'
    }, function (err, res) {
        pm.test("Response time is " + res.responseTime, function (){
            pm.expect(err).to.equal(null);
            pm.expect(res).to.have.property('code', 200);
            responseTimes.push(res.responseTime);
        });
        if (i < iterations - 1) {
            i++;
            setTimeout(sendRequest, delay);
        } 
        else {
            percentile90ResponseTime = quantile(responseTimes, 90);
            pm.test("90 percentile response time " + percentile90ResponseTime + " is lower than " + maximumResponseTime + ", the number of iterations is " + iterations, function () {
                    pm.expect(percentile90ResponseTime).to.be.below(maximumResponseTime);
                });
        }
    });
}
sendRequest();
function sortNumber(a,b) {
    return a - b;
}
function quantile(array, percentile) {
    array.sort(sortNumber);
    index = percentile/100. * (array.length-1);
    if (Math.floor(index) == index) {
     result = array[index];
    } else {
        j = Math.floor(index)
        fraction = index - j;
        result = array[j] + (array[j+1] - array[j]) * fraction;
    }
    return result;
}