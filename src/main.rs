use num::complex::Complex;    // <1>从num软件包的Complex子模块中,导人复数数字类型Complex°

fn calculate_mandelbrot(      // <2>在输出空间(一个行和列的网格)和芒德布罗集的空间范围(靠近 (0,0) 的连续区域)之间执行转换。

  max_iters: usize,           // <3>如果一个值在达到最大迭代次数之前都没有逃逸,那么就认为此值是在芒德布罗集的范围之内的。
  x_min: f64,                 // <4>这4个参数指定了我们要搜索的集合成员的空间范围°
  x_max: f64,                 // <4>
  y_min: f64,                 // <4>
  y_max: f64,                 // <4>
  width: usize,               // <5>这两个参数表示输出空间的大小，单位是像素
  height: usize,              // <5>
  ) -> Vec<Vec<usize>> {

  let mut rows: Vec<_> = Vec::with_capacity(width); // <6>创建一个容器，用于容纳每行的数据。
  for img_y in 0..height {                          // <7>按行迭代允许我们逐行输出要输出的内容

    let mut row: Vec<usize> = Vec::with_capacity(height);
    for img_x in 0..width {

      let x_percent = img_x as f64 / width as f64;
      let y_percent = img_y as f64 / height as f64;
      let cx = x_min + (x_max - x_min) * x_percent; // <8>计算我们在输出中要覆盖的空间比例，并将其转换为搜索空间中的点。
      let cy = y_min + (y_max - y_min) * y_percent; // <8>
      let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
      row.push(escaped_at);
    }

    rows.push(row);
  }
  rows
}

fn mandelbrot_at_point(   // <9>在每个像素上调用 (每个像素对应要输出的行和列的值)。
  cx: f64,
  cy: f64,
  max_iters: usize,
  ) -> usize {
  let mut z = Complex { re: 0.0, im: 0.0 };       // <10>将一个复数初始化为原点的值，实部 (re)和虚部 (im)都为0.0。
  let c = Complex::new(cx, cy);                   // <11>使用作为函数参数提供的坐标值来初始化一个复数

  for i in 0..=max_iters {
    if z.norm() > 2.0 {                           // <12>检查逃选条件。z.norm0用于计算到原点 (0,0) 的距离返回-个复数的绝对值。
      return i;
    }
    z = z * z + c;                                // <13>反复改变z的值，用来检查 c 是否位于芒德布罗集之内。
  }
  max_iters                                       // <14>因为i在这里不再存在，所以在检查失败后我们返回 max iters。
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
  for row in escape_vals {
    let mut line = String::with_capacity(row.len());
    for column in row {
      let val = match column {
        0..=2 => ' ',
        3..=5 => '.',
        6..=10 => '•',
        11..=30 => '*',
        31..=100 => '+',
        101..=200 => 'x',
        201..=400 => '$',
        401..=700 => '#',
        _ => '%',
      };

      line.push(val);
    }
    println!("{}", line);
  }
}

fn main() {
  
  let mandelbrot = calculate_mandelbrot(1000, 
                              -2.0, 1.0, -1.0, 1.0, 
                              100, 24);

  render_mandelbrot(mandelbrot);
}
