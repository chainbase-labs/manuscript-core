package com.chainbase.common.cached;

import com.google.common.cache.CacheBuilder;
import java.io.Serializable;
import java.util.concurrent.ConcurrentMap;
import java.util.function.Function;

public class LRUCache<K, V> implements Serializable {

  private static final long serialVersionUID = -9221578054952959859L;
  private final ConcurrentMap<K, V> cached;

  public LRUCache(long maxSize) {
    this.cached = CacheBuilder.newBuilder().maximumSize(maxSize).<K, V>build().asMap();
  }

  public void put(K key, V value) {
    cached.put(key, value);
  }

  public V computeIfAbsent(K key, Function<? super K, ? extends V> callback) {
    return cached.computeIfAbsent(key, callback);
  }


  public V get(K key) {
    return cached.get(key);
  }

  public boolean containsKey(K key) {
    return cached.containsKey(key);
  }
}
