window.addEventListener('load', function load() {
    window.removeEventListener('load', load);

    import('../wasm_build/index').then(({ App }) => {
        const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
        const { width, height } = canvas.getBoundingClientRect();

        const app = App.new(width, height);
        for (let i = 0; i < 100; i++) {
            app.add_boid(Math.random() * width, Math.random() * height);
        }

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
