hello-name = Hello, { $name }!
my-name-and-age = 
  My name is {$name} and {$age ->
    [0] ???
    [one] I'm one year old!
    *[other] {$age} years old
  }
