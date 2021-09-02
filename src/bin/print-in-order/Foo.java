import java.util.concurrent.atomic.AtomicInteger;

public class Foo {

    public static void main(String[] args) throws InterruptedException {
        Foo foo = new Foo();
        Thread t1 = new Thread(() -> {
            try {
                foo.first(() -> System.out.print("first"));
            } catch (InternalError | InterruptedException e) {
            }
        });
        Thread t2 = new Thread(() -> {
                try {
                    foo.second(() -> System.out.print("second"));
                }catch (InternalError | InterruptedException e) {
                }
        });
        Thread t3 = new Thread(() -> {
            try {
                foo.third(() -> System.out.print("thrid"));
            } catch (InternalError | InterruptedException e) {
            }
        });
        t1.start();
        t2.start();
        t3.start();

        t1.join();
        t2.join();
        t3.join();
    }

    final AtomicInteger count = new AtomicInteger();

    public Foo() {
    }

    public void first(Runnable printFirst) throws InterruptedException {
        // printFirst.run() outputs "first". Do not change or remove this line.
        printFirst.run();
        count.set(1);
    }

    public void second(Runnable printSecond) throws InterruptedException {
        while (!count.compareAndSet(1, -1)) {}
        // printSecond.run() outputs "second". Do not change or remove this line.
        printSecond.run();
        count.set(2);
    }

    public void third(Runnable printThird) throws InterruptedException {
        while (!count.compareAndSet(2, -1)) {}
        // printThird.run() outputs "third". Do not change or remove this line.
        printThird.run();
    }
}
