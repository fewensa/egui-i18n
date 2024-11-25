hello-name = 你好 {$name}!
my-name-and-age =
  我的名字是 {$name} {$age ->
    [0] 年龄 ???
    [1] 我今年 1 岁了!
    *[other] , 我 {$age} 岁了
  }
