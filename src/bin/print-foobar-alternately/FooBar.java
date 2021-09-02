public class FooBar {
    public static void main(String []args) throws InterruptedException {
        FooBar fb = new FooBar(3);
        Thread f = new Thread(() -> {
                try {
                    fb.foo(() -> System.out.print("foo"));
                } catch (InternalError | InterruptedException e) {
                }
        });

        Thread b = new Thread(() -> {
                try {
                    fb.bar(() -> System.out.print("bar"));
                }catch (InternalError | InterruptedException e) {
                }
        });

        f.start();
        b.start();

        f.join();
        b.join();
    }

    private int n;

    public FooBar(int n) {
        this.n = n;
    }

    private java.util.concurrent.atomic.AtomicInteger b = new java.util.concurrent.atomic.AtomicInteger();

    public void foo(Runnable printFoo) throws InterruptedException {
        for (int i = 0; i < n; i++) {
            while (!b.compareAndSet(0, -1)) {
            }
            // printFoo.run() outputs "foo". Do not change or remove this line.
            printFoo.run();
            b.set(1);
        }
    }

    public void bar(Runnable printBar) throws InterruptedException {
        for (int i = 0; i < n; i++) {
            while (!b.compareAndSet(1, -1)) {
            }
            // printBar.run() outputs "bar". Do not change or remove this line.
            printBar.run();
            b.set(0);
        }
    }
}
