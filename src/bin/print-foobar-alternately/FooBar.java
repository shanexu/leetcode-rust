import java.util.concurrent.locks.Condition;
import java.util.concurrent.locks.ReentrantLock;

public class FooBar {
    public static void main(String[] args) throws InterruptedException {
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
            } catch (InternalError | InterruptedException e) {
            }
        });

        f.start();
        b.start();

        f.join();
        b.join();
    }

    private int n;
    private ReentrantLock lock = new ReentrantLock();
    private Condition fcond = lock.newCondition();
    private Condition bcond = lock.newCondition();
    private boolean foo = true;

    public FooBar(int n) {
        this.n = n;
    }

    public void foo(Runnable printFoo) throws InterruptedException {
        lock.lock();
        for (int i = 0; i < n; i++) {
            while (!foo) {
                fcond.await();
            }
            foo = !foo;
            // printFoo.run() outputs "foo". Do not change or remove this line.
            printFoo.run();
            bcond.signal();
        }
        lock.unlock();
    }

    public void bar(Runnable printBar) throws InterruptedException {
        lock.lock();
        for (int i = 0; i < n; i++) {
            while (foo) {
                bcond.await();
            }
            foo = !foo;
            // printBar.run() outputs "bar". Do not change or remove this line.
            printBar.run();
            fcond.signal();
        }
        lock.unlock();
    }
}
