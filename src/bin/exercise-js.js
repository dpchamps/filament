// Do some javascripty stuff
const go = () => {
    const now = Date.now();
    let count = 0
    logInfo(new Date(), count);

    while((Date.now() - now) < 500) {
        count ++
    }

    logInfo(new Date(), count);

    function* x(){
        yield 1;
        yield 2;
        yield 3;
        yield 4;
    }

    logInfo([...x()]);
}
(Promise.resolve("a")).then(x => {
    logInfo(`From promise: ${x}`);
})
go();
logInfo(Object.entries(this));