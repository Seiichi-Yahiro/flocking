window.addEventListener('load', function load() {
    window.removeEventListener('load', load);

    import('../wasm_build').then(({ BoidPool }) => {
        const boidPool = BoidPool.new(800, 600);
        boidPool.add_boid(100, 100);
        boidPool.add_boid(250, 250);

        window.addEventListener('beforeunload', () => {
            boidPool.free();
        });

        const renderLoop = () => {
            boidPool.update();
            boidPool.render();
            requestAnimationFrame(renderLoop);
        };

        requestAnimationFrame(renderLoop);
    });
});
