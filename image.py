import mandelbrot


centre = mandelbrot.Complex(-0.4605111, 0.56011)
max_iter = 1000
scale = 1.25e-2
rate = 0.9
res = [3440, 3440]
frames = 1

mandelbrot.mandelbrot_video(centre, scale, rate, res, frames, max_iter)
