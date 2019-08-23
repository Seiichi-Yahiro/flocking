window.addEventListener('load', function load() {
    window.removeEventListener('load', load);

    import('./wasm').then(({ app }) => {
        const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
        const { width, height } = canvas.getBoundingClientRect();

        for (let i = 0; i < 0; i++) {
            app.add_boid(Math.random() * width, Math.random() * height);
        }

        canvas.addEventListener('mousemove', ({ clientX, clientY }: MouseEvent) => {
            const { left, top } = canvas.getBoundingClientRect();
            app.set_mouse_pos(clientX - left, clientY - top);
        });

        canvas.addEventListener('click', ({ clientX, clientY }) => {
            const { left, top } = canvas.getBoundingClientRect();
            app.add_boid(clientX - left, clientY - top);
        });

        const renderLoop = () => {
            app.update();
            app.render();
            requestAnimationFrame(renderLoop);
        };

        requestAnimationFrame(renderLoop);
    });
});
