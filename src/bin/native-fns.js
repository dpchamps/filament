logInfo(Object.keys(this));
const x = passCallback(() => "Result");
logInfo(x, x());
const y = callCbFromNative(() => "Result");
logInfo(y);