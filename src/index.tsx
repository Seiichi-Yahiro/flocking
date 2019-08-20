window.addEventListener('load', function load() {
    window.removeEventListener('load', load);

    import('../wasm_build/index').then(({ App }) => {
        const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
        const { width, height } = canvas.getBoundingClientRect();

        const app = App.new(width, height);
        app.add_boid(100, 100);
        app.add_boid(250, 250);

        window.addEventListener('beforeunload', () => {
            app.free();
        });

        canvas.addEventListener('mousemove', ({ clientX, clientY }: MouseEvent) => {
            const { left, top } = canvas.getBoundingClientRect();
            app.set_mouse_pos(clientX - left, clientY - top);
        });

        const renderLoop = () => {
            app.update();
            app.render();
            requestAnimationFrame(renderLoop);
        };

        requestAnimationFrame(renderLoop);
    });
});
