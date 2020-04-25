# Carbon

Carbon is a prototype to make local file sharing easy. It isn't
secure, it's a prototype.

The idea is that you can run `carbon-server` to listen on the local
network. Another computer can list listening servers using `carbon-ls`
and send files to local server using `carbon-cp`.

To send a file to Alice's (Computer A) from Bob's (Computer B), it
might look similar to this:


## Computer A
~~~
$ ./carbon-server
~~~

## Comptuer B

~~~
$ ./carbon-ls

alice@192.168.0.100

$ ./carbon-cp ~/cat.jpg alice

Sending cat.jpg to 192.168.0.100
cat.jpg sent
~~~
