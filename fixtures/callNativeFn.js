'use strict'
const x = passCallback(() => "Hello");
logInfo(x);
logInfo(x());

try{
    passCallback();
}catch(e){
    logInfo(e);
}
