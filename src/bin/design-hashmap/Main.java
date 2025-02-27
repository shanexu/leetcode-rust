public class Main {
    public static void main(String[] args) {
        MyHashMap map = new MyHashMap();
        map.put(1, 10);
        System.out.println(map.get(1));
        map.put(1, 11);
        System.out.println(map.get(1));
        map.remove(1);
        System.out.println(map.get(1));
        map.put(1, 10);
        map.put(10001, 11);
        System.out.println(map.get(1));
        System.out.println(map.get(10001));
        map.remove(1);
        System.out.println(map.get(1));
        System.out.println(map.get(10001));
    }
}

class MyHashMap {
    private static final int CAPACITY = 16384;
    private static final int MASK = CAPACITY - 1;
    private Node[] nodes;

    public MyHashMap() {
        this.nodes = new Node[CAPACITY];
    }

    public void put(int key, int value) {
        int r = key & MASK;
        Node curr = nodes[r];
        while (curr != null) {
            if (curr.key == key) {
                curr.value = value;
                break;
            }
            curr = curr.next;
        }
        if (curr == null) {
            curr = new Node(key, value);
            curr.next = nodes[r];
            nodes[r] = curr;
        }
    }

    public int get(int key) {
        int r = key & MASK;
        Node curr = nodes[r];
        while (curr != null) {
            if (curr.key == key) {
                return curr.value;
            }
            curr = curr.next;
        }
        return -1;
    }

    public void remove(int key) {
        int r = key & MASK;
        Node dummy = new Node(-1, -1);
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
}

class Node {
    int key;
    int value;
    Node next;
    public Node(int key, int value) {
        this.key = key;
        this.value = value;
    }
}
