# 308 REEDPIPES
Having been a reed pipe enthusiast for a long time now, our cousin cobbled together a little numerically controlled machine that will enable him to carry out a serial production of reed pipes and make a business out of it. However, he would like a software so that he can design his pipes himself. . .
So, we have to create a program for him that, starting from the pipe’s radius (in cm) with abscissas 0, 5, 10, 15 and 20 cm, and using cubic splines, displays the radii of n points that are evenly distributed along the pipe. In order to simplify the debugging process, we will also display the resolved linear system’s vector result in order to obtain the spline.
## How to build
```sh
$ make re
```

## Examples
```sh
$> ./308reedpipes -h
USAGE
    ./308reedpipes r0 r5 r10 r15 r20 n
DESCRIPTION
    r0      radius (in cm) of pipe at the 0cm abscissa
    r5      radius (in cm) of pipe at the 5cm abscissa
    r10     radius (in cm) of pipe at the 10cm abscissa
    r15     radius (in cm) of pipe at the 15cm abscissa
    r20     radius (in cm) of pipe at the 20cm abscissa
    n       number of points needed to display the radius
```

```sh
./308reedpipes 1.5 2 2 2 5 11
vector result: [0.0, 0.0, 0.0, 0.2, 0.0]
abscissa: 0.0 cm    radius: 1.5 cm
abscissa: 2.0 cm    radius: 1.7 cm
abscissa: 4.0 cm    radius: 1.9 cm
abscissa: 6.0 cm    radius: 2.1 cm
abscissa: 8.0 cm    radius: 2.1 cm
abscissa: 10.0 cm   radius: 2.0 cm
abscissa: 12.0 cm   radius: 1.8 cm
abscissa: 14.0 cm   radius: 1.8 cm
abscissa: 16.0 cm   radius: 2.4 cm
abscissa: 18.0 cm   radius: 3.5 cm
abscissa: 20.0 cm   radius: 5.0 cm
```

```sh
/308reedpipes 2 3 2 4 5 13
vector result: [0.0, -0.2, 0.3, -0.1, 0.0]
abscissa: 0.0 cm    radius: 2.0 cm
abscissa: 1.7 cm    radius: 2.6 cm
abscissa: 3.3 cm    radius: 3.0 cm
abscissa: 5.0 cm    radius: 3.0 cm
abscissa: 6.7 cm    radius: 2.6 cm
abscissa: 8.3 cm    radius: 2.2 cm
abscissa: 10.0 cm   radius: 2.0 cm
abscissa: 11.7 cm   radius: 2.4 cm
abscissa: 13.3 cm   radius: 3.2 cm
abscissa: 15.0 cm   radius: 4.0 cm
abscissa: 16.7 cm   radius: 4.5 cm
abscissa: 18.3 cm   radius: 4.8 cm
abscissa: 20.0 cm   radius: 5.0 cm
```