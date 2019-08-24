import React from 'react';
import ReactDom from 'react-dom';

const init = async () => {
    const { Simulation, Settings } = await import('../wasm_build/index');
    const Interface = (await import('./tsx/Interface')).default;

    ReactDom.render(<Interface />, document.getElementById('root'));

    const simulation = Simulation.new();

    window.addEventListener('beforeunload', () => {
        simulation.free();
    });

    const canvas = document.getElementById('canvas')! as HTMLCanvasElement;
    const { width, height } = canvas.getBoundingClientRect();

    for (let i = 0; i < 0; i++) {
        simulation.add_boid(Math.random() * width, Math.random() * height);
    }

    canvas.addEventListener('mousemove', ({ clientX, clientY }: MouseEvent) => {
        const { left, top } = canvas.getBoundingClientRect();
        Settings.set_mouse_position(clientX - left, clientY - top);
    });

    canvas.addEventListener('click', ({ clientX, clientY }) => {
        const { left, top } = canvas.getBoundingClientRect();
        simulation.add_boid(clientX - left, clientY - top);
    });

    const renderLoop = () => {
        simulation.tick();
        requestAnimationFrame(renderLoop);
    };

    requestAnimationFrame(renderLoop);
};

window.addEventListener('load', function load() {
    window.removeEventListener('load', load);
    init();
});
