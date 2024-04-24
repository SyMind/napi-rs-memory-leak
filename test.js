const { QueryEngine } = require('./index.js');

const finalizationRegistry = new FinalizationRegistry(key => {
    console.log('gc: ', key)
});

function successToGC() {
    const foo = {}
    const queryEngine = new QueryEngine(() => {
        // console.log(queryEngine);
    });
    finalizationRegistry.register(foo, 'foo1');
    finalizationRegistry.register(queryEngine, 'queryEngine1');
}

function failedToGC() {
    const foo = {}
    const queryEngine = new QueryEngine(() => {
        // This console.log will prevent queryEngine from being garbage collected
        // because it creates a closure over the queryEngine variable.
        console.log(queryEngine);
    });
    finalizationRegistry.register(foo, 'foo2');
    finalizationRegistry.register(queryEngine, 'queryEngine2');
}

successToGC();
gc();

failedToGC();
gc();
