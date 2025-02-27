public class Main {
    public static void main(String[] args) {
    }
}

class MyHashSet {
    private static final int CAPACITY = 16384;
    private static final int MASK = CAPACITY - 1;
    private Node[] nodes;

    public MyHashSet() {
        nodes = new Node[CAPACITY];
    }

    public void add(int key) {
        int r = key & MASK;
        Node curr = nodes[r];
        while (curr != null) {
            if (curr.key == key) {
                return;
            }
            curr = curr.next;
        }
        curr = new Node(key);
        curr.next = nodes[r];
        nodes[r] = curr;
    }

    public void remove(int key) {
        int r = key & MASK;
        Node dummy = new Node(-1);
        Node prev = dummy;
        Node curr = nodes[r];
        prev.next = curr;
        while (curr != null) {
            if (curr.key == key) {
                prev.next = curr.next;
                break;
            } else {
                prev = curr;
                curr = curr.next;
            }
        }
        nodes[r] = dummy.next;
    }

    public boolean contains(int key) {
        int r = key & MASK;
        Node curr = nodes[r];
        while (curr != null) {
            if (curr.key == key) {
                return true;
            }
            curr = curr.next;
        }
        return false;
    }
}

class Node {
    int key;
    Node next;

    public Node(int key) {
        this.key = key;
    }
}
