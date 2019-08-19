window.addEventListener('load', function load() {
    window.removeEventListener('load', load);

    import('../wasm_build').then(({ BoidPool }) => {
        const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
        const { width, height } = canvas.getBoundingClientRect();

        const boidPool = BoidPool.new(width, height);
        boidPool.add_boid(100, 100);
        boidPool.add_boid(250, 250);

        window.addEventListener('beforeunload', () => {
            boidPool.free();
        });

        canvas.addEventListener('mousemove', ({ clientX, clientY }: MouseEvent) => {
            const { left, top } = canvas.getBoundingClientRect();
            boidPool.set_mouse_pos(clientX - left, clientY - top);
        });

        const renderLoop = () => {
            boidPool.update();
            boidPool.render();
            requestAnimationFrame(renderLoop);
        };

        requestAnimationFrame(renderLoop);
    });
});
