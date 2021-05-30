# with Rust:
`./target/release/rust_posts`
`ab -c 1000 -n 15000 http://192.168.1.190:3000/posts`

>Server Port:            3000
>Server Hostname:        IP_OF_RPI
>Document Path:          /posts
>Document Length:        519 bytes
>
>Concurrency Level:      1000
>Time taken for tests:   9.123 seconds
>Complete requests:      15000
>Failed requests:        0
>Total transferred:      9420000 bytes
>HTML transferred:       7785000 bytes
>Requests per second:    1644.21 [#/sec] (mean)
>Time per request:       608.194 [ms] (mean)
>Time per request:       0.608 [ms] (mean, across all concurrent requests)
>Transfer rate:          1008.37 [Kbytes/sec] received
>
>Connection Times (ms)
>              min  mean[+/-sd] median   max
>Connect:        0   48 290.5      1    3143
>Processing:    14  543  99.2    581     808
>Waiting:        5  543  99.2    581     807
>Total:         16  592 311.1    583    3779
>
>Percentage of the requests served within a certain time (ms)
>  50%    583
>  66%    594
>  75%    601
>  80%    604
>  90%    611
>  95%    620
>  98%   1579
>  99%   1810
> 100%   3779 (longest request)




# With nodejs + pm2
`pm2 start main.js -i max`
`ab -c 1000 -n 15000 http://192.168.1.190:3000/posts` (from another PC on the LAN)

>Server Software:
>Server Hostname:        IP_OF_RPI
>Server Port:            3001
>
>Document Path:          /posts
>Document Length:        519 bytes
>
>Concurrency Level:      1000
>Time taken for tests:   28.638 seconds
>Complete requests:      15000
>Failed requests:        0
>Total transferred:      10635000 bytes
>HTML transferred:       7785000 bytes
>Requests per second:    523.78 [#/sec] (mean)
>Time per request:       1909.214 [ms] (mean)
>Time per request:       1.909 [ms] (mean, across all concurrent requests)
>Transfer rate:          362.65 [Kbytes/sec] received
>
>Connection Times (ms)
>              min  mean[+/-sd] median   max
>Connect:        0   51 310.2      1    3152
>Processing:    59 1807 340.1   1843    2504
>Waiting:       47 1802 339.5   1837    2475
>Total:         70 1858 452.3   1859    5148
>
>Percentage of the requests served within a certain time (ms)
>  50%   1859
>  66%   2007
>  75%   2057
>  80%   2082
>  90%   2232
>  95%   2327
>  98%   2651
>  99%   3130
> 100%   5148 (longest request)
>