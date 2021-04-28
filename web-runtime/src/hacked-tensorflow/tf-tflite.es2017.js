/**
 * @license
 * Copyright 2021 Google LLC. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * =============================================================================
 */
(function (global, factory) {
	typeof exports === 'object' && typeof module !== 'undefined' ? factory(exports, require('@tensorflow/tfjs-core')) :
	typeof define === 'function' && define.amd ? define(['exports', '@tensorflow/tfjs-core'], factory) :
	(global = typeof globalThis !== 'undefined' ? globalThis : global || self, factory(global.tf = global.tf || {}, global.tf));
}(this, (function (exports, tfjsCore) { 'use strict';

	var commonjsGlobal = typeof globalThis !== 'undefined' ? globalThis : typeof window !== 'undefined' ? window : typeof global !== 'undefined' ? global : typeof self !== 'undefined' ? self : {};

	/*

	 Copyright The Closure Library Authors.
	 SPDX-License-Identifier: Apache-2.0
	*/
	var $jscomp = $jscomp || {};
	$jscomp.scope = {};
	$jscomp.arrayIteratorImpl = function (a) {
	    var b = 0;
	    return function () {
	        return b < a.length ? {
	            done: !1,
	            value: a[b++]
	        } : {
	            done: !0
	        }
	    }
	};
	$jscomp.arrayIterator = function (a) {
	    return {
	        next: $jscomp.arrayIteratorImpl(a)
	    }
	};
	$jscomp.ASSUME_ES5 = !1;
	$jscomp.ASSUME_NO_NATIVE_MAP = !1;
	$jscomp.ASSUME_NO_NATIVE_SET = !1;
	$jscomp.SIMPLE_FROUND_POLYFILL = !1;
	$jscomp.ISOLATE_POLYFILLS = !1;
	$jscomp.FORCE_POLYFILL_PROMISE = !1;
	$jscomp.FORCE_POLYFILL_PROMISE_WHEN_NO_UNHANDLED_REJECTION = !1;
	$jscomp.defineProperty = $jscomp.ASSUME_ES5 || "function" == typeof Object.defineProperties ? Object.defineProperty : function (a, b, c) {
	    if (a == Array.prototype || a == Object.prototype) return a;
	    a[b] = c.value;
	    return a
	};
	$jscomp.getGlobal = function (a) {
	    a = ["object" == typeof globalThis && globalThis, a, "object" == typeof window && window, "object" == typeof self && self, "object" == typeof commonjsGlobal && commonjsGlobal];
	    for (var b = 0; b < a.length; ++b) {
	        var c = a[b];
	        if (c && c.Math == Math) return c
	    }
	    throw Error("Cannot find global object");
	};
	$jscomp.global = $jscomp.getGlobal(commonjsGlobal);
	$jscomp.IS_SYMBOL_NATIVE = "function" === typeof Symbol && "symbol" === typeof Symbol("x");
	$jscomp.TRUST_ES6_POLYFILLS = !$jscomp.ISOLATE_POLYFILLS || $jscomp.IS_SYMBOL_NATIVE;
	$jscomp.polyfills = {};
	$jscomp.propertyToPolyfillSymbol = {};
	$jscomp.POLYFILL_PREFIX = "$jscp$";
	$jscomp.polyfill = function (a, b, c, d) {
	    b && ($jscomp.ISOLATE_POLYFILLS ? $jscomp.polyfillIsolated(a, b, c, d) : $jscomp.polyfillUnisolated(a, b, c, d));
	};
	$jscomp.polyfillUnisolated = function (a, b) {
	    var c = $jscomp.global;
	    a = a.split(".");
	    for (var d = 0; d < a.length - 1; d++) {
	        var e = a[d];
	        if (!(e in c)) return;
	        c = c[e];
	    }
	    a = a[a.length - 1];
	    d = c[a];
	    b = b(d);
	    b != d && null != b && $jscomp.defineProperty(c, a, {
	        configurable: !0,
	        writable: !0,
	        value: b
	    });
	};
	$jscomp.polyfillIsolated = function (a, b, c) {
	    var d = a.split(".");
	    a = 1 === d.length;
	    var e = d[0];
	    e = !a && e in $jscomp.polyfills ? $jscomp.polyfills : $jscomp.global;
	    for (var f = 0; f < d.length - 1; f++) {
	        var g = d[f];
	        if (!(g in e)) return;
	        e = e[g];
	    }
	    d = d[d.length - 1];
	    c = $jscomp.IS_SYMBOL_NATIVE && "es6" === c ? e[d] : null;
	    b = b(c);
	    null != b && (a ? $jscomp.defineProperty($jscomp.polyfills, d, {
	        configurable: !0,
	        writable: !0,
	        value: b
	    }) : b !== c && (void 0 === $jscomp.propertyToPolyfillSymbol[d] && ($jscomp.propertyToPolyfillSymbol[d] = $jscomp.IS_SYMBOL_NATIVE ? $jscomp.global.Symbol(d) :
	        $jscomp.POLYFILL_PREFIX + d), $jscomp.defineProperty(e, $jscomp.propertyToPolyfillSymbol[d], {
	        configurable: !0,
	        writable: !0,
	        value: b
	    })));
	};
	$jscomp.initSymbol = function () {};
	$jscomp.polyfill("Symbol", function (a) {
	    if (a) return a;
	    var b = function (e, f) {
	        this.$jscomp$symbol$id_ = e;
	        $jscomp.defineProperty(this, "description", {
	            configurable: !0,
	            writable: !0,
	            value: f
	        });
	    };
	    b.prototype.toString = function () {
	        return this.$jscomp$symbol$id_
	    };
	    var c = 0,
	        d = function (e) {
	            if (this instanceof d) throw new TypeError("Symbol is not a constructor");
	            return new b("jscomp_symbol_" + (e || "") + "_" + c++, e)
	        };
	    return d
	}, "es6", "es3");
	$jscomp.polyfill("Symbol.iterator", function (a) {
	        if (a) return a;
	        a = Symbol("Symbol.iterator");
	        for (var b = "Array Int8Array Uint8Array Uint8ClampedArray Int16Array Uint16Array Int32Array Uint32Array Float32Array Float64Array".split(" "), c = 0; c < b.length; c++) {
	            var d = $jscomp.global[b[c]];
	            "function" === typeof d && "function" != typeof d.prototype[a] && $jscomp.defineProperty(d.prototype, a, {
	                configurable: !0,
	                writable: !0,
	                value: function () {
	                    return $jscomp.iteratorPrototype($jscomp.arrayIteratorImpl(this))
	                }
	            });
	        }
	        return a
	    }, "es6",
	    "es3");
	$jscomp.iteratorPrototype = function (a) {
	    a = {
	        next: a
	    };
	    a[Symbol.iterator] = function () {
	        return this
	    };
	    return a
	};
	$jscomp.createTemplateTagFirstArg = function (a) {
	    return a.raw = a
	};
	$jscomp.createTemplateTagFirstArgWithRaw = function (a, b) {
	    a.raw = b;
	    return a
	};
	$jscomp.makeIterator = function (a) {
	    var b = "undefined" != typeof Symbol && Symbol.iterator && a[Symbol.iterator];
	    return b ? b.call(a) : $jscomp.arrayIterator(a)
	};
	$jscomp.underscoreProtoCanBeSet = function () {
	    var a = {
	            a: !0
	        },
	        b = {};
	    try {
	        return b.__proto__ = a, b.a
	    } catch (c) {}
	    return !1
	};
	$jscomp.setPrototypeOf = $jscomp.TRUST_ES6_POLYFILLS && "function" == typeof Object.setPrototypeOf ? Object.setPrototypeOf : $jscomp.underscoreProtoCanBeSet() ? function (a, b) {
	    a.__proto__ = b;
	    if (a.__proto__ !== b) throw new TypeError(a + " is not extensible");
	    return a
	} : null;
	$jscomp.generator = {};
	$jscomp.generator.ensureIteratorResultIsObject_ = function (a) {
	    if (!(a instanceof Object)) throw new TypeError("Iterator result " + a + " is not an object");
	};
	$jscomp.generator.Context = function () {
	    this.isRunning_ = !1;
	    this.yieldAllIterator_ = null;
	    this.yieldResult = void 0;
	    this.nextAddress = 1;
	    this.finallyAddress_ = this.catchAddress_ = 0;
	    this.finallyContexts_ = this.abruptCompletion_ = null;
	};
	$jscomp.generator.Context.prototype.start_ = function () {
	    if (this.isRunning_) throw new TypeError("Generator is already running");
	    this.isRunning_ = !0;
	};
	$jscomp.generator.Context.prototype.stop_ = function () {
	    this.isRunning_ = !1;
	};
	$jscomp.generator.Context.prototype.jumpToErrorHandler_ = function () {
	    this.nextAddress = this.catchAddress_ || this.finallyAddress_;
	};
	$jscomp.generator.Context.prototype.next_ = function (a) {
	    this.yieldResult = a;
	};
	$jscomp.generator.Context.prototype.throw_ = function (a) {
	    this.abruptCompletion_ = {
	        exception: a,
	        isException: !0
	    };
	    this.jumpToErrorHandler_();
	};
	$jscomp.generator.Context.prototype.return = function (a) {
	    this.abruptCompletion_ = {
	        return: a
	    };
	    this.nextAddress = this.finallyAddress_;
	};
	$jscomp.generator.Context.prototype.jumpThroughFinallyBlocks = function (a) {
	    this.abruptCompletion_ = {
	        jumpTo: a
	    };
	    this.nextAddress = this.finallyAddress_;
	};
	$jscomp.generator.Context.prototype.yield = function (a, b) {
	    this.nextAddress = b;
	    return {
	        value: a
	    }
	};
	$jscomp.generator.Context.prototype.yieldAll = function (a, b) {
	    a = $jscomp.makeIterator(a);
	    var c = a.next();
	    $jscomp.generator.ensureIteratorResultIsObject_(c);
	    if (c.done) this.yieldResult = c.value, this.nextAddress = b;
	    else return this.yieldAllIterator_ = a, this.yield(c.value, b)
	};
	$jscomp.generator.Context.prototype.jumpTo = function (a) {
	    this.nextAddress = a;
	};
	$jscomp.generator.Context.prototype.jumpToEnd = function () {
	    this.nextAddress = 0;
	};
	$jscomp.generator.Context.prototype.setCatchFinallyBlocks = function (a, b) {
	    this.catchAddress_ = a;
	    void 0 != b && (this.finallyAddress_ = b);
	};
	$jscomp.generator.Context.prototype.setFinallyBlock = function (a) {
	    this.catchAddress_ = 0;
	    this.finallyAddress_ = a || 0;
	};
	$jscomp.generator.Context.prototype.leaveTryBlock = function (a, b) {
	    this.nextAddress = a;
	    this.catchAddress_ = b || 0;
	};
	$jscomp.generator.Context.prototype.enterCatchBlock = function (a) {
	    this.catchAddress_ = a || 0;
	    a = this.abruptCompletion_.exception;
	    this.abruptCompletion_ = null;
	    return a
	};
	$jscomp.generator.Context.prototype.enterFinallyBlock = function (a, b, c) {
	    c ? this.finallyContexts_[c] = this.abruptCompletion_ : this.finallyContexts_ = [this.abruptCompletion_];
	    this.catchAddress_ = a || 0;
	    this.finallyAddress_ = b || 0;
	};
	$jscomp.generator.Context.prototype.leaveFinallyBlock = function (a, b) {
	    b = this.finallyContexts_.splice(b || 0)[0];
	    if (b = this.abruptCompletion_ = this.abruptCompletion_ || b) {
	        if (b.isException) return this.jumpToErrorHandler_();
	        void 0 != b.jumpTo && this.finallyAddress_ < b.jumpTo ? (this.nextAddress = b.jumpTo, this.abruptCompletion_ = null) : this.nextAddress = this.finallyAddress_;
	    } else this.nextAddress = a;
	};
	$jscomp.generator.Context.prototype.forIn = function (a) {
	    return new $jscomp.generator.Context.PropertyIterator(a)
	};
	$jscomp.generator.Context.PropertyIterator = function (a) {
	    this.object_ = a;
	    this.properties_ = [];
	    for (var b in a) this.properties_.push(b);
	    this.properties_.reverse();
	};
	$jscomp.generator.Context.PropertyIterator.prototype.getNext = function () {
	    for (; 0 < this.properties_.length;) {
	        var a = this.properties_.pop();
	        if (a in this.object_) return a
	    }
	    return null
	};
	$jscomp.generator.Engine_ = function (a) {
	    this.context_ = new $jscomp.generator.Context;
	    this.program_ = a;
	};
	$jscomp.generator.Engine_.prototype.next_ = function (a) {
	    this.context_.start_();
	    if (this.context_.yieldAllIterator_) return this.yieldAllStep_(this.context_.yieldAllIterator_.next, a, this.context_.next_);
	    this.context_.next_(a);
	    return this.nextStep_()
	};
	$jscomp.generator.Engine_.prototype.return_ = function (a) {
	    this.context_.start_();
	    var b = this.context_.yieldAllIterator_;
	    if (b) return this.yieldAllStep_("return" in b ? b["return"] : function (c) {
	        return {
	            value: c,
	            done: !0
	        }
	    }, a, this.context_.return);
	    this.context_.return(a);
	    return this.nextStep_()
	};
	$jscomp.generator.Engine_.prototype.throw_ = function (a) {
	    this.context_.start_();
	    if (this.context_.yieldAllIterator_) return this.yieldAllStep_(this.context_.yieldAllIterator_["throw"], a, this.context_.next_);
	    this.context_.throw_(a);
	    return this.nextStep_()
	};
	$jscomp.generator.Engine_.prototype.yieldAllStep_ = function (a, b, c) {
	    try {
	        var d = a.call(this.context_.yieldAllIterator_, b);
	        $jscomp.generator.ensureIteratorResultIsObject_(d);
	        if (!d.done) return this.context_.stop_(), d;
	        var e = d.value;
	    } catch (f) {
	        return this.context_.yieldAllIterator_ = null, this.context_.throw_(f), this.nextStep_()
	    }
	    this.context_.yieldAllIterator_ = null;
	    c.call(this.context_, e);
	    return this.nextStep_()
	};
	$jscomp.generator.Engine_.prototype.nextStep_ = function () {
	    for (; this.context_.nextAddress;) try {
	        var a = this.program_(this.context_);
	        if (a) return this.context_.stop_(), {
	            value: a.value,
	            done: !1
	        }
	    } catch (b) {
	        this.context_.yieldResult = void 0, this.context_.throw_(b);
	    }
	    this.context_.stop_();
	    if (this.context_.abruptCompletion_) {
	        a = this.context_.abruptCompletion_;
	        this.context_.abruptCompletion_ = null;
	        if (a.isException) throw a.exception;
	        return {
	            value: a.return,
	            done: !0
	        }
	    }
	    return {
	        value: void 0,
	        done: !0
	    }
	};
	$jscomp.generator.Generator_ = function (a) {
	    this.next = function (b) {
	        return a.next_(b)
	    };
	    this.throw = function (b) {
	        return a.throw_(b)
	    };
	    this.return = function (b) {
	        return a.return_(b)
	    };
	    this[Symbol.iterator] = function () {
	        return this
	    };
	};
	$jscomp.generator.createGenerator = function (a, b) {
	    b = new $jscomp.generator.Generator_(new $jscomp.generator.Engine_(b));
	    $jscomp.setPrototypeOf && a.prototype && $jscomp.setPrototypeOf(b, a.prototype);
	    return b
	};
	$jscomp.checkStringArgs = function (a, b, c) {
	    if (null == a) throw new TypeError("The 'this' value for String.prototype." + c + " must not be null or undefined");
	    if (b instanceof RegExp) throw new TypeError("First argument to String.prototype." + c + " must not be a regular expression");
	    return a + ""
	};
	$jscomp.polyfill("String.prototype.endsWith", function (a) {
	    return a ? a : function (b, c) {
	        var d = $jscomp.checkStringArgs(this, b, "endsWith");
	        b += "";
	        void 0 === c && (c = d.length);
	        c = Math.max(0, Math.min(c | 0, d.length));
	        for (var e = b.length; 0 < e && 0 < c;)
	            if (d[--c] != b[--e]) return !1;
	        return 0 >= e
	    }
	}, "es6", "es3");
	$jscomp.findInternal = function (a, b, c) {
	    a instanceof String && (a = String(a));
	    for (var d = a.length, e = 0; e < d; e++) {
	        var f = a[e];
	        if (b.call(c, f, e, a)) return {
	            i: e,
	            v: f
	        }
	    }
	    return {
	        i: -1,
	        v: void 0
	    }
	};
	$jscomp.polyfill("String.prototype.startsWith", function (a) {
	    return a ? a : function (b, c) {
	        var d = $jscomp.checkStringArgs(this, b, "startsWith");
	        b += "";
	        var e = d.length,
	            f = b.length;
	        c = Math.max(0, Math.min(c | 0, d.length));
	        for (var g = 0; g < f && c < e;)
	            if (d[c++] != b[g++]) return !1;
	        return g >= f
	    }
	}, "es6", "es3");
	$jscomp.polyfill("String.prototype.repeat", function (a) {
	    return a ? a : function (b) {
	        var c = $jscomp.checkStringArgs(this, null, "repeat");
	        if (0 > b || 1342177279 < b) throw new RangeError("Invalid count value");
	        b |= 0;
	        for (var d = ""; b;)
	            if (b & 1 && (d += c), b >>>= 1) c += c;
	        return d
	    }
	}, "es6", "es3");
	$jscomp.polyfill("Object.setPrototypeOf", function (a) {
	    return a || $jscomp.setPrototypeOf
	}, "es6", "es5");
	$jscomp.owns = function (a, b) {
	    return Object.prototype.hasOwnProperty.call(a, b)
	};
	$jscomp.assign = $jscomp.TRUST_ES6_POLYFILLS && "function" == typeof Object.assign ? Object.assign : function (a, b) {
	    for (var c = 1; c < arguments.length; c++) {
	        var d = arguments[c];
	        if (d)
	            for (var e in d) $jscomp.owns(d, e) && (a[e] = d[e]);
	    }
	    return a
	};
	$jscomp.polyfill("Object.assign", function (a) {
	    return a || $jscomp.assign
	}, "es6", "es3");
	$jscomp.polyfill("Promise", function (a) {
	    function b() {
	        this.batch_ = null;
	    }

	    function c(h) {
	        return h instanceof f ? h : new f(function (k) {
	            k(h);
	        })
	    }
	    if (a && (!($jscomp.FORCE_POLYFILL_PROMISE || $jscomp.FORCE_POLYFILL_PROMISE_WHEN_NO_UNHANDLED_REJECTION && "undefined" === typeof $jscomp.global.PromiseRejectionEvent) || !$jscomp.global.Promise || -1 === $jscomp.global.Promise.toString().indexOf("[native code]"))) return a;
	    b.prototype.asyncExecute = function (h) {
	        if (null == this.batch_) {
	            this.batch_ = [];
	            var k = this;
	            this.asyncExecuteFunction(function () {
	                k.executeBatch_();
	            });
	        }
	        this.batch_.push(h);
	    };
	    var d = $jscomp.global.setTimeout;
	    b.prototype.asyncExecuteFunction = function (h) {
	        d(h, 0);
	    };
	    b.prototype.executeBatch_ = function () {
	        for (; this.batch_ && this.batch_.length;) {
	            var h = this.batch_;
	            this.batch_ = [];
	            for (var k = 0; k < h.length; ++k) {
	                var l = h[k];
	                h[k] = null;
	                try {
	                    l();
	                } catch (m) {
	                    this.asyncThrow_(m);
	                }
	            }
	        }
	        this.batch_ = null;
	    };
	    b.prototype.asyncThrow_ = function (h) {
	        this.asyncExecuteFunction(function () {
	            throw h;
	        });
	    };
	    var e = {
	            PENDING: 0,
	            FULFILLED: 1,
	            REJECTED: 2
	        },
	        f = function (h) {
	            this.state_ = e.PENDING;
	            this.result_ = void 0;
	            this.onSettledCallbacks_ = [];
	            this.isRejectionHandled_ = !1;
	            var k = this.createResolveAndReject_();
	            try {
	                h(k.resolve, k.reject);
	            } catch (l) {
	                k.reject(l);
	            }
	        };
	    f.prototype.createResolveAndReject_ = function () {
	        function h(m) {
	            return function (n) {
	                l || (l = !0, m.call(k, n));
	            }
	        }
	        var k = this,
	            l = !1;
	        return {
	            resolve: h(this.resolveTo_),
	            reject: h(this.reject_)
	        }
	    };
	    f.prototype.resolveTo_ = function (h) {
	        if (h === this) this.reject_(new TypeError("A Promise cannot resolve to itself"));
	        else if (h instanceof f) this.settleSameAsPromise_(h);
	        else {
	            a: switch (typeof h) {
	                case "object":
	                    var k =
	                        null != h;
	                    break a;
	                case "function":
	                    k = !0;
	                    break a;
	                default:
	                    k = !1;
	            }
	            k ? this.resolveToNonPromiseObj_(h) : this.fulfill_(h);
	        }
	    };
	    f.prototype.resolveToNonPromiseObj_ = function (h) {
	        var k = void 0;
	        try {
	            k = h.then;
	        } catch (l) {
	            this.reject_(l);
	            return
	        }
	        "function" == typeof k ? this.settleSameAsThenable_(k, h) : this.fulfill_(h);
	    };
	    f.prototype.reject_ = function (h) {
	        this.settle_(e.REJECTED, h);
	    };
	    f.prototype.fulfill_ = function (h) {
	        this.settle_(e.FULFILLED, h);
	    };
	    f.prototype.settle_ = function (h, k) {
	        if (this.state_ != e.PENDING) throw Error("Cannot settle(" + h +
	            ", " + k + "): Promise already settled in state" + this.state_);
	        this.state_ = h;
	        this.result_ = k;
	        this.state_ === e.REJECTED && this.scheduleUnhandledRejectionCheck_();
	        this.executeOnSettledCallbacks_();
	    };
	    f.prototype.scheduleUnhandledRejectionCheck_ = function () {
	        var h = this;
	        d(function () {
	            if (h.notifyUnhandledRejection_()) {
	                var k = $jscomp.global.console;
	                "undefined" !== typeof k && k.error(h.result_);
	            }
	        }, 1);
	    };
	    f.prototype.notifyUnhandledRejection_ = function () {
	        if (this.isRejectionHandled_) return !1;
	        var h = $jscomp.global.CustomEvent,
	            k =
	            $jscomp.global.Event,
	            l = $jscomp.global.dispatchEvent;
	        if ("undefined" === typeof l) return !0;
	        "function" === typeof h ? h = new h("unhandledrejection", {
	            cancelable: !0
	        }) : "function" === typeof k ? h = new k("unhandledrejection", {
	            cancelable: !0
	        }) : (h = $jscomp.global.document.createEvent("CustomEvent"), h.initCustomEvent("unhandledrejection", !1, !0, h));
	        h.promise = this;
	        h.reason = this.result_;
	        return l(h)
	    };
	    f.prototype.executeOnSettledCallbacks_ = function () {
	        if (null != this.onSettledCallbacks_) {
	            for (var h = 0; h < this.onSettledCallbacks_.length; ++h) g.asyncExecute(this.onSettledCallbacks_[h]);
	            this.onSettledCallbacks_ = null;
	        }
	    };
	    var g = new b;
	    f.prototype.settleSameAsPromise_ = function (h) {
	        var k = this.createResolveAndReject_();
	        h.callWhenSettled_(k.resolve, k.reject);
	    };
	    f.prototype.settleSameAsThenable_ = function (h, k) {
	        var l = this.createResolveAndReject_();
	        try {
	            h.call(k, l.resolve, l.reject);
	        } catch (m) {
	            l.reject(m);
	        }
	    };
	    f.prototype.then = function (h, k) {
	        function l(q, p) {
	            return "function" == typeof q ? function (t) {
	                try {
	                    m(q(t));
	                } catch (u) {
	                    n(u);
	                }
	            } : p
	        }
	        var m, n, r = new f(function (q, p) {
	            m = q;
	            n = p;
	        });
	        this.callWhenSettled_(l(h, m), l(k, n));
	        return r
	    };
	    f.prototype.catch = function (h) {
	        return this.then(void 0, h)
	    };
	    f.prototype.callWhenSettled_ = function (h, k) {
	        function l() {
	            switch (m.state_) {
	                case e.FULFILLED:
	                    h(m.result_);
	                    break;
	                case e.REJECTED:
	                    k(m.result_);
	                    break;
	                default:
	                    throw Error("Unexpected state: " + m.state_);
	            }
	        }
	        var m = this;
	        null == this.onSettledCallbacks_ ? g.asyncExecute(l) : this.onSettledCallbacks_.push(l);
	        this.isRejectionHandled_ = !0;
	    };
	    f.resolve = c;
	    f.reject = function (h) {
	        return new f(function (k, l) {
	            l(h);
	        })
	    };
	    f.race = function (h) {
	        return new f(function (k, l) {
	            for (var m =
	                    $jscomp.makeIterator(h), n = m.next(); !n.done; n = m.next()) c(n.value).callWhenSettled_(k, l);
	        })
	    };
	    f.all = function (h) {
	        var k = $jscomp.makeIterator(h),
	            l = k.next();
	        return l.done ? c([]) : new f(function (m, n) {
	            function r(t) {
	                return function (u) {
	                    q[t] = u;
	                    p--;
	                    0 == p && m(q);
	                }
	            }
	            var q = [],
	                p = 0;
	            do q.push(void 0), p++, c(l.value).callWhenSettled_(r(q.length - 1), n), l = k.next(); while (!l.done)
	        })
	    };
	    return f
	}, "es6", "es3");
	$jscomp.checkEs6ConformanceViaProxy = function () {
	    try {
	        var a = {},
	            b = Object.create(new $jscomp.global.Proxy(a, {
	                get: function (c, d, e) {
	                    return c == a && "q" == d && e == b
	                }
	            }));
	        return !0 === b.q
	    } catch (c) {
	        return !1
	    }
	};
	$jscomp.USE_PROXY_FOR_ES6_CONFORMANCE_CHECKS = !1;
	$jscomp.ES6_CONFORMANCE = $jscomp.USE_PROXY_FOR_ES6_CONFORMANCE_CHECKS && $jscomp.checkEs6ConformanceViaProxy();
	$jscomp.polyfill("WeakMap", function (a) {
	    function b() {
	        if (!a || !Object.seal) return !1;
	        try {
	            var l = Object.seal({}),
	                m = Object.seal({}),
	                n = new a([
	                    [l, 2],
	                    [m, 3]
	                ]);
	            if (2 != n.get(l) || 3 != n.get(m)) return !1;
	            n.delete(l);
	            n.set(m, 4);
	            return !n.has(l) && 4 == n.get(m)
	        } catch (r) {
	            return !1
	        }
	    }

	    function c() {}

	    function d(l) {
	        var m = typeof l;
	        return "object" === m && null !== l || "function" === m
	    }

	    function e(l) {
	        if (!$jscomp.owns(l, g)) {
	            var m = new c;
	            $jscomp.defineProperty(l, g, {
	                value: m
	            });
	        }
	    }

	    function f(l) {
	        if (!$jscomp.ISOLATE_POLYFILLS) {
	            var m = Object[l];
	            m && (Object[l] =
	                function (n) {
	                    if (n instanceof c) return n;
	                    Object.isExtensible(n) && e(n);
	                    return m(n)
	                });
	        }
	    }
	    if ($jscomp.USE_PROXY_FOR_ES6_CONFORMANCE_CHECKS) {
	        if (a && $jscomp.ES6_CONFORMANCE) return a
	    } else if (b()) return a;
	    var g = "$jscomp_hidden_" + Math.random();
	    f("freeze");
	    f("preventExtensions");
	    f("seal");
	    var h = 0,
	        k = function (l) {
	            this.id_ = (h += Math.random() + 1).toString();
	            if (l) {
	                l = $jscomp.makeIterator(l);
	                for (var m; !(m = l.next()).done;) m = m.value, this.set(m[0], m[1]);
	            }
	        };
	    k.prototype.set = function (l, m) {
	        if (!d(l)) throw Error("Invalid WeakMap key");
	        e(l);
	        if (!$jscomp.owns(l, g)) throw Error("WeakMap key fail: " + l);
	        l[g][this.id_] = m;
	        return this
	    };
	    k.prototype.get = function (l) {
	        return d(l) && $jscomp.owns(l, g) ? l[g][this.id_] : void 0
	    };
	    k.prototype.has = function (l) {
	        return d(l) && $jscomp.owns(l, g) && $jscomp.owns(l[g], this.id_)
	    };
	    k.prototype.delete = function (l) {
	        return d(l) && $jscomp.owns(l, g) && $jscomp.owns(l[g], this.id_) ? delete l[g][this.id_] : !1
	    };
	    return k
	}, "es6", "es3");
	$jscomp.MapEntry = function () {};
	$jscomp.polyfill("Map", function (a) {
	    function b() {
	        if ($jscomp.ASSUME_NO_NATIVE_MAP || !a || "function" != typeof a || !a.prototype.entries || "function" != typeof Object.seal) return !1;
	        try {
	            var k = Object.seal({
	                    x: 4
	                }),
	                l = new a($jscomp.makeIterator([
	                    [k, "s"]
	                ]));
	            if ("s" != l.get(k) || 1 != l.size || l.get({
	                    x: 4
	                }) || l.set({
	                    x: 4
	                }, "t") != l || 2 != l.size) return !1;
	            var m = l.entries(),
	                n = m.next();
	            if (n.done || n.value[0] != k || "s" != n.value[1]) return !1;
	            n = m.next();
	            return n.done || 4 != n.value[0].x || "t" != n.value[1] || !m.next().done ? !1 : !0
	        } catch (r) {
	            return !1
	        }
	    }
	    if ($jscomp.USE_PROXY_FOR_ES6_CONFORMANCE_CHECKS) {
	        if (a && $jscomp.ES6_CONFORMANCE) return a
	    } else if (b()) return a;
	    var c = new WeakMap,
	        d = function (k) {
	            this.data_ = {};
	            this.head_ = g();
	            this.size = 0;
	            if (k) {
	                k = $jscomp.makeIterator(k);
	                for (var l; !(l = k.next()).done;) l = l.value, this.set(l[0], l[1]);
	            }
	        };
	    d.prototype.set = function (k, l) {
	        k = 0 === k ? 0 : k;
	        var m = e(this, k);
	        m.list || (m.list = this.data_[m.id] = []);
	        m.entry ? m.entry.value = l : (m.entry = {
	                next: this.head_,
	                previous: this.head_.previous,
	                head: this.head_,
	                key: k,
	                value: l
	            }, m.list.push(m.entry),
	            this.head_.previous.next = m.entry, this.head_.previous = m.entry, this.size++);
	        return this
	    };
	    d.prototype.delete = function (k) {
	        k = e(this, k);
	        return k.entry && k.list ? (k.list.splice(k.index, 1), k.list.length || delete this.data_[k.id], k.entry.previous.next = k.entry.next, k.entry.next.previous = k.entry.previous, k.entry.head = null, this.size--, !0) : !1
	    };
	    d.prototype.clear = function () {
	        this.data_ = {};
	        this.head_ = this.head_.previous = g();
	        this.size = 0;
	    };
	    d.prototype.has = function (k) {
	        return !!e(this, k).entry
	    };
	    d.prototype.get = function (k) {
	        return (k =
	            e(this, k).entry) && k.value
	    };
	    d.prototype.entries = function () {
	        return f(this, function (k) {
	            return [k.key, k.value]
	        })
	    };
	    d.prototype.keys = function () {
	        return f(this, function (k) {
	            return k.key
	        })
	    };
	    d.prototype.values = function () {
	        return f(this, function (k) {
	            return k.value
	        })
	    };
	    d.prototype.forEach = function (k, l) {
	        for (var m = this.entries(), n; !(n = m.next()).done;) n = n.value, k.call(l, n[1], n[0], this);
	    };
	    d.prototype[Symbol.iterator] = d.prototype.entries;
	    var e = function (k, l) {
	            var m = l && typeof l;
	            "object" == m || "function" == m ? c.has(l) ? m = c.get(l) :
	                (m = "" + ++h, c.set(l, m)) : m = "p_" + l;
	            var n = k.data_[m];
	            if (n && $jscomp.owns(k.data_, m))
	                for (k = 0; k < n.length; k++) {
	                    var r = n[k];
	                    if (l !== l && r.key !== r.key || l === r.key) return {
	                        id: m,
	                        list: n,
	                        index: k,
	                        entry: r
	                    }
	                }
	            return {
	                id: m,
	                list: n,
	                index: -1,
	                entry: void 0
	            }
	        },
	        f = function (k, l) {
	            var m = k.head_;
	            return $jscomp.iteratorPrototype(function () {
	                if (m) {
	                    for (; m.head != k.head_;) m = m.previous;
	                    for (; m.next != m.head;) return m = m.next, {
	                        done: !1,
	                        value: l(m)
	                    };
	                    m = null;
	                }
	                return {
	                    done: !0,
	                    value: void 0
	                }
	            })
	        },
	        g = function () {
	            var k = {};
	            return k.previous = k.next = k.head = k
	        },
	        h = 0;
	    return d
	}, "es6", "es3");
	var goog = goog || {};
	goog.global = commonjsGlobal || self;
	goog.exportPath_ = function (a, b, c, d) {
	    a = a.split(".");
	    d = d || goog.global;
	    a[0] in d || "undefined" == typeof d.execScript || d.execScript("var " + a[0]);
	    for (var e; a.length && (e = a.shift());)
	        if (a.length || void 0 === b) d = d[e] && d[e] !== Object.prototype[e] ? d[e] : d[e] = {};
	        else if (!c && goog.isObject(b) && goog.isObject(d[e]))
	        for (var f in b) b.hasOwnProperty(f) && (d[e][f] = b[f]);
	    else d[e] = b;
	};
	goog.define = function (a, b) {
	    return b
	};
	goog.FEATURESET_YEAR = 2012;
	goog.DEBUG = !0;
	goog.LOCALE = "en";
	goog.TRUSTED_SITE = !0;
	goog.DISALLOW_TEST_ONLY_CODE = !goog.DEBUG;
	goog.ENABLE_CHROME_APP_SAFE_SCRIPT_LOADING = !1;
	goog.provide = function (a) {
	    if (goog.isInModuleLoader_()) throw Error("goog.provide cannot be used within a module.");
	    goog.constructNamespace_(a);
	};
	goog.constructNamespace_ = function (a, b, c) {
	    goog.exportPath_(a, b, c);
	};
	goog.getScriptNonce = function (a) {
	    if (a && a != goog.global) return goog.getScriptNonce_(a.document);
	    null === goog.cspNonce_ && (goog.cspNonce_ = goog.getScriptNonce_(goog.global.document));
	    return goog.cspNonce_
	};
	goog.NONCE_PATTERN_ = /^[\w+/_-]+[=]{0,2}$/;
	goog.cspNonce_ = null;
	goog.getScriptNonce_ = function (a) {
	    return (a = a.querySelector && a.querySelector("script[nonce]")) && (a = a.nonce || a.getAttribute("nonce")) && goog.NONCE_PATTERN_.test(a) ? a : ""
	};
	goog.VALID_MODULE_RE_ = /^[a-zA-Z_$][a-zA-Z0-9._$]*$/;
	goog.module = function (a) {
	    if ("string" !== typeof a || !a || -1 == a.search(goog.VALID_MODULE_RE_)) throw Error("Invalid module identifier");
	    if (!goog.isInGoogModuleLoader_()) throw Error("Module " + a + " has been loaded incorrectly. Note, modules cannot be loaded as normal scripts. They require some kind of pre-processing step. You're likely trying to load a module via a script tag or as a part of a concatenated bundle without rewriting the module. For more info see: https://github.com/google/closure-library/wiki/goog.module:-an-ES6-module-like-alternative-to-goog.provide.");
	    if (goog.moduleLoaderState_.moduleName) throw Error("goog.module may only be called once per module.");
	    goog.moduleLoaderState_.moduleName = a;
	};
	goog.module.get = function (a) {
	    return goog.module.getInternal_(a)
	};
	goog.module.getInternal_ = function () {
	    return null
	};
	goog.ModuleType = {
	    ES6: "es6",
	    GOOG: "goog"
	};
	goog.moduleLoaderState_ = null;
	goog.isInModuleLoader_ = function () {
	    return goog.isInGoogModuleLoader_() || goog.isInEs6ModuleLoader_()
	};
	goog.isInGoogModuleLoader_ = function () {
	    return !!goog.moduleLoaderState_ && goog.moduleLoaderState_.type == goog.ModuleType.GOOG
	};
	goog.isInEs6ModuleLoader_ = function () {
	    if (goog.moduleLoaderState_ && goog.moduleLoaderState_.type == goog.ModuleType.ES6) return !0;
	    var a = goog.global.$jscomp;
	    return a ? "function" != typeof a.getCurrentModulePath ? !1 : !!a.getCurrentModulePath() : !1
	};
	goog.module.declareLegacyNamespace = function () {
	    goog.moduleLoaderState_.declareLegacyNamespace = !0;
	};
	goog.declareModuleId = function (a) {
	    if (goog.moduleLoaderState_) goog.moduleLoaderState_.moduleName = a;
	    else {
	        var b = goog.global.$jscomp;
	        if (!b || "function" != typeof b.getCurrentModulePath) throw Error('Module with namespace "' + a + '" has been loaded incorrectly.');
	        b = b.require(b.getCurrentModulePath());
	        goog.loadedModules_[a] = {
	            exports: b,
	            type: goog.ModuleType.ES6,
	            moduleId: a
	        };
	    }
	};
	goog.setTestOnly = function (a) {
	    if (goog.DISALLOW_TEST_ONLY_CODE) throw a = a || "", Error("Importing test-only code into non-debug environment" + (a ? ": " + a : "."));
	};
	goog.forwardDeclare = function () {};
	goog.getObjectByName = function (a, b) {
	    a = a.split(".");
	    b = b || goog.global;
	    for (var c = 0; c < a.length; c++)
	        if (b = b[a[c]], null == b) return null;
	    return b
	};
	goog.addDependency = function () {};
	goog.useStrictRequires = !1;
	goog.ENABLE_DEBUG_LOADER = !0;
	goog.logToConsole_ = function (a) {
	    goog.global.console && goog.global.console.error(a);
	};
	goog.require = function () {};
	goog.requireType = function () {
	    return {}
	};
	goog.basePath = "";
	goog.nullFunction = function () {};
	goog.abstractMethod = function () {
	    throw Error("unimplemented abstract method");
	};
	goog.addSingletonGetter = function (a) {
	    a.instance_ = void 0;
	    a.getInstance = function () {
	        if (a.instance_) return a.instance_;
	        goog.DEBUG && (goog.instantiatedSingletons_[goog.instantiatedSingletons_.length] = a);
	        return a.instance_ = new a
	    };
	};
	goog.instantiatedSingletons_ = [];
	goog.LOAD_MODULE_USING_EVAL = !0;
	goog.SEAL_MODULE_EXPORTS = goog.DEBUG;
	goog.loadedModules_ = {};
	goog.DEPENDENCIES_ENABLED = !1;
	goog.TRANSPILE = "detect";
	goog.ASSUME_ES_MODULES_TRANSPILED = !1;
	goog.TRANSPILE_TO_LANGUAGE = "";
	goog.TRANSPILER = "transpile.js";
	goog.TRUSTED_TYPES_POLICY_NAME = "goog";
	goog.hasBadLetScoping = null;
	goog.loadModule = function (a) {
	    var b = goog.moduleLoaderState_;
	    try {
	        goog.moduleLoaderState_ = {
	            moduleName: "",
	            declareLegacyNamespace: !1,
	            type: goog.ModuleType.GOOG
	        };
	        var c = {},
	            d = c;
	        if ("function" === typeof a) d = a.call(void 0, d);
	        else if ("string" === typeof a) d = goog.loadModuleFromSource_.call(void 0, d, a);
	        else throw Error("Invalid module definition");
	        var e = goog.moduleLoaderState_.moduleName;
	        if ("string" === typeof e && e) goog.moduleLoaderState_.declareLegacyNamespace ? goog.constructNamespace_(e, d, c !== d) : goog.SEAL_MODULE_EXPORTS &&
	            Object.seal && "object" == typeof d && null != d && Object.seal(d), goog.loadedModules_[e] = {
	                exports: d,
	                type: goog.ModuleType.GOOG,
	                moduleId: goog.moduleLoaderState_.moduleName
	            };
	        else throw Error('Invalid module name "' + e + '"');
	    } finally {
	        goog.moduleLoaderState_ = b;
	    }
	};
	goog.loadModuleFromSource_ = function (a, b) {
	    eval(goog.CLOSURE_EVAL_PREFILTER_.createScript(b));
	    return a
	};
	goog.normalizePath_ = function (a) {
	    a = a.split("/");
	    for (var b = 0; b < a.length;) "." == a[b] ? a.splice(b, 1) : b && ".." == a[b] && a[b - 1] && ".." != a[b - 1] ? a.splice(--b, 2) : b++;
	    return a.join("/")
	};
	goog.loadFileSync_ = function (a) {
	    if (goog.global.CLOSURE_LOAD_FILE_SYNC) return goog.global.CLOSURE_LOAD_FILE_SYNC(a);
	    try {
	        var b = new goog.global.XMLHttpRequest;
	        b.open("get", a, !1);
	        b.send();
	        return 0 == b.status || 200 == b.status ? b.responseText : null
	    } catch (c) {
	        return null
	    }
	};
	goog.transpile_ = function (a, b, c) {
	    var d = goog.global.$jscomp;
	    d || (goog.global.$jscomp = d = {});
	    var e = d.transpile;
	    if (!e) {
	        var f = goog.basePath + goog.TRANSPILER,
	            g = goog.loadFileSync_(f);
	        if (g) {
	            (function () {
	                (0, eval)(g + "\n//# sourceURL=" + f);
	            }).call(goog.global);
	            if (goog.global.$gwtExport && goog.global.$gwtExport.$jscomp && !goog.global.$gwtExport.$jscomp.transpile) throw Error('The transpiler did not properly export the "transpile" method. $gwtExport: ' + JSON.stringify(goog.global.$gwtExport));
	            goog.global.$jscomp.transpile =
	                goog.global.$gwtExport.$jscomp.transpile;
	            d = goog.global.$jscomp;
	            e = d.transpile;
	        }
	    }
	    if (!e) {
	        var h = " requires transpilation but no transpiler was found.";
	        h += ' Please add "//javascript/closure:transpiler" as a data dependency to ensure it is included.';
	        e = d.transpile = function (k, l) {
	            goog.logToConsole_(l + h);
	            return k
	        };
	    }
	    return e(a, b, c)
	};
	goog.typeOf = function (a) {
	    var b = typeof a;
	    return "object" != b ? b : a ? Array.isArray(a) ? "array" : b : "null"
	};
	goog.isArrayLike = function (a) {
	    var b = goog.typeOf(a);
	    return "array" == b || "object" == b && "number" == typeof a.length
	};
	goog.isDateLike = function (a) {
	    return goog.isObject(a) && "function" == typeof a.getFullYear
	};
	goog.isObject = function (a) {
	    var b = typeof a;
	    return "object" == b && null != a || "function" == b
	};
	goog.getUid = function (a) {
	    return Object.prototype.hasOwnProperty.call(a, goog.UID_PROPERTY_) && a[goog.UID_PROPERTY_] || (a[goog.UID_PROPERTY_] = ++goog.uidCounter_)
	};
	goog.hasUid = function (a) {
	    return !!a[goog.UID_PROPERTY_]
	};
	goog.removeUid = function (a) {
	    null !== a && "removeAttribute" in a && a.removeAttribute(goog.UID_PROPERTY_);
	    try {
	        delete a[goog.UID_PROPERTY_];
	    } catch (b) {}
	};
	goog.UID_PROPERTY_ = "closure_uid_" + (1E9 * Math.random() >>> 0);
	goog.uidCounter_ = 0;
	goog.cloneObject = function (a) {
	    var b = goog.typeOf(a);
	    if ("object" == b || "array" == b) {
	        if ("function" === typeof a.clone) return a.clone();
	        b = "array" == b ? [] : {};
	        for (var c in a) b[c] = goog.cloneObject(a[c]);
	        return b
	    }
	    return a
	};
	goog.bindNative_ = function (a, b, c) {
	    return a.call.apply(a.bind, arguments)
	};
	goog.bindJs_ = function (a, b, c) {
	    if (!a) throw Error();
	    if (2 < arguments.length) {
	        var d = Array.prototype.slice.call(arguments, 2);
	        return function () {
	            var e = Array.prototype.slice.call(arguments);
	            Array.prototype.unshift.apply(e, d);
	            return a.apply(b, e)
	        }
	    }
	    return function () {
	        return a.apply(b, arguments)
	    }
	};
	goog.bind = function (a, b, c) {
	    Function.prototype.bind && -1 != Function.prototype.bind.toString().indexOf("native code") ? goog.bind = goog.bindNative_ : goog.bind = goog.bindJs_;
	    return goog.bind.apply(null, arguments)
	};
	goog.partial = function (a, b) {
	    var c = Array.prototype.slice.call(arguments, 1);
	    return function () {
	        var d = c.slice();
	        d.push.apply(d, arguments);
	        return a.apply(this, d)
	    }
	};
	goog.mixin = function (a, b) {
	    for (var c in b) a[c] = b[c];
	};
	goog.now = function () {
	    return Date.now()
	};
	goog.globalEval = function (a) {
	    (0, eval)(a);
	};
	goog.getCssName = function (a, b) {
	    if ("." == String(a).charAt(0)) throw Error('className passed in goog.getCssName must not start with ".". You passed: ' + a);
	    var c = function (e) {
	            return goog.cssNameMapping_[e] || e
	        },
	        d = function (e) {
	            e = e.split("-");
	            for (var f = [], g = 0; g < e.length; g++) f.push(c(e[g]));
	            return f.join("-")
	        };
	    d = goog.cssNameMapping_ ? "BY_WHOLE" == goog.cssNameMappingStyle_ ? c : d : function (e) {
	        return e
	    };
	    a = b ? a + "-" + d(b) : d(a);
	    return goog.global.CLOSURE_CSS_NAME_MAP_FN ? goog.global.CLOSURE_CSS_NAME_MAP_FN(a) : a
	};
	goog.setCssNameMapping = function (a, b) {
	    goog.cssNameMapping_ = a;
	    goog.cssNameMappingStyle_ = b;
	};
	goog.getMsg = function (a, b, c) {
	    c && c.html && (a = a.replace(/</g, "&lt;"));
	    c && c.unescapeHtmlEntities && (a = a.replace(/&lt;/g, "<").replace(/&gt;/g, ">").replace(/&apos;/g, "'").replace(/&quot;/g, '"').replace(/&amp;/g, "&"));
	    b && (a = a.replace(/\{\$([^}]+)}/g, function (d, e) {
	        return null != b && e in b ? b[e] : d
	    }));
	    return a
	};
	goog.getMsgWithFallback = function (a) {
	    return a
	};
	goog.exportSymbol = function (a, b, c) {
	    goog.exportPath_(a, b, !0, c);
	};
	goog.exportProperty = function (a, b, c) {
	    a[b] = c;
	};
	goog.inherits = function (a, b) {
	    function c() {}
	    c.prototype = b.prototype;
	    a.superClass_ = b.prototype;
	    a.prototype = new c;
	    a.prototype.constructor = a;
	    a.base = function (d, e, f) {
	        for (var g = Array(arguments.length - 2), h = 2; h < arguments.length; h++) g[h - 2] = arguments[h];
	        return b.prototype[e].apply(d, g)
	    };
	};
	goog.scope = function (a) {
	    if (goog.isInModuleLoader_()) throw Error("goog.scope is not supported within a module.");
	    a.call(goog.global);
	};
	goog.defineClass = function (a, b) {
	    var c = b.constructor,
	        d = b.statics;
	    c && c != Object.prototype.constructor || (c = function () {
	        throw Error("cannot instantiate an interface (no constructor defined).");
	    });
	    c = goog.defineClass.createSealingConstructor_(c, a);
	    a && goog.inherits(c, a);
	    delete b.constructor;
	    delete b.statics;
	    goog.defineClass.applyProperties_(c.prototype, b);
	    null != d && (d instanceof Function ? d(c) : goog.defineClass.applyProperties_(c, d));
	    return c
	};
	goog.defineClass.SEAL_CLASS_INSTANCES = goog.DEBUG;
	goog.defineClass.createSealingConstructor_ = function (a) {
	    return goog.defineClass.SEAL_CLASS_INSTANCES ? function () {
	        var b = a.apply(this, arguments) || this;
	        b[goog.UID_PROPERTY_] = b[goog.UID_PROPERTY_];
	        return b
	    } : a
	};
	goog.defineClass.OBJECT_PROTOTYPE_FIELDS_ = "constructor hasOwnProperty isPrototypeOf propertyIsEnumerable toLocaleString toString valueOf".split(" ");
	goog.defineClass.applyProperties_ = function (a, b) {
	    for (var c in b) Object.prototype.hasOwnProperty.call(b, c) && (a[c] = b[c]);
	    for (var d = 0; d < goog.defineClass.OBJECT_PROTOTYPE_FIELDS_.length; d++) c = goog.defineClass.OBJECT_PROTOTYPE_FIELDS_[d], Object.prototype.hasOwnProperty.call(b, c) && (a[c] = b[c]);
	};
	goog.identity_ = function (a) {
	    return a
	};
	goog.createTrustedTypesPolicy = function (a) {
	    var b = null,
	        c = goog.global.trustedTypes;
	    if (!c || !c.createPolicy) return b;
	    try {
	        b = c.createPolicy(a, {
	            createHTML: goog.identity_,
	            createScript: goog.identity_,
	            createScriptURL: goog.identity_
	        });
	    } catch (d) {
	        goog.logToConsole_(d.message);
	    }
	    return b
	};
	var jspb = {},
	    module$contents$jspb$ConstBinaryMessage_ConstBinaryMessage = function () {};
	module$contents$jspb$ConstBinaryMessage_ConstBinaryMessage.prototype.toDebugString = function () {};
	module$contents$jspb$ConstBinaryMessage_ConstBinaryMessage.prototype.toDebugStringInternal = function () {};
	jspb.ConstBinaryMessage = module$contents$jspb$ConstBinaryMessage_ConstBinaryMessage;
	jspb.BinaryMessage = function () {};
	jspb.ScalarFieldType = void 0;
	jspb.RepeatedFieldType = void 0;
	jspb.AnyFieldType = void 0;
	jspb.arith = {};
	jspb.arith.UInt64 = function (a, b) {
	    this.lo = a;
	    this.hi = b;
	};
	jspb.arith.UInt64.prototype.cmp = function (a) {
	    return this.hi < a.hi || this.hi == a.hi && this.lo < a.lo ? -1 : this.hi == a.hi && this.lo == a.lo ? 0 : 1
	};
	jspb.arith.UInt64.prototype.rightShift = function () {
	    return new jspb.arith.UInt64((this.lo >>> 1 | (this.hi & 1) << 31) >>> 0, this.hi >>> 1 >>> 0)
	};
	jspb.arith.UInt64.prototype.leftShift = function () {
	    return new jspb.arith.UInt64(this.lo << 1 >>> 0, (this.hi << 1 | this.lo >>> 31) >>> 0)
	};
	jspb.arith.UInt64.prototype.msb = function () {
	    return !!(this.hi & 2147483648)
	};
	jspb.arith.UInt64.prototype.lsb = function () {
	    return !!(this.lo & 1)
	};
	jspb.arith.UInt64.prototype.zero = function () {
	    return 0 == this.lo && 0 == this.hi
	};
	jspb.arith.UInt64.prototype.add = function (a) {
	    return new jspb.arith.UInt64((this.lo + a.lo & 4294967295) >>> 0 >>> 0, ((this.hi + a.hi & 4294967295) >>> 0) + (4294967296 <= this.lo + a.lo ? 1 : 0) >>> 0)
	};
	jspb.arith.UInt64.prototype.sub = function (a) {
	    return new jspb.arith.UInt64((this.lo - a.lo & 4294967295) >>> 0 >>> 0, ((this.hi - a.hi & 4294967295) >>> 0) - (0 > this.lo - a.lo ? 1 : 0) >>> 0)
	};
	jspb.arith.UInt64.mul32x32 = function (a, b) {
	    var c = a & 65535;
	    a >>>= 16;
	    var d = b & 65535,
	        e = b >>> 16;
	    b = c * d + 65536 * (c * e & 65535) + 65536 * (a * d & 65535);
	    for (c = a * e + (c * e >>> 16) + (a * d >>> 16); 4294967296 <= b;) b -= 4294967296, c += 1;
	    return new jspb.arith.UInt64(b >>> 0, c >>> 0)
	};
	jspb.arith.UInt64.prototype.mul = function (a) {
	    var b = jspb.arith.UInt64.mul32x32(this.lo, a);
	    a = jspb.arith.UInt64.mul32x32(this.hi, a);
	    a.hi = a.lo;
	    a.lo = 0;
	    return b.add(a)
	};
	jspb.arith.UInt64.prototype.div = function (a) {
	    if (0 == a) return [];
	    for (var b = new jspb.arith.UInt64(0, 0), c = new jspb.arith.UInt64(this.lo, this.hi), d = new jspb.arith.UInt64(a, 0), e = new jspb.arith.UInt64(1, 0); !d.msb();) d = d.leftShift(), e = e.leftShift();
	    for (; !e.zero();) 0 >= d.cmp(c) && (b = b.add(e), c = c.sub(d)), d = d.rightShift(), e = e.rightShift();
	    return [b, c]
	};
	jspb.arith.UInt64.prototype.toString = function () {
	    for (var a = "", b = this; !b.zero();) {
	        b = b.div(10);
	        var c = b[0];
	        a = b[1].lo + a;
	        b = c;
	    }
	    "" == a && (a = "0");
	    return a
	};
	jspb.arith.UInt64.fromString = function (a) {
	    for (var b = new jspb.arith.UInt64(0, 0), c = new jspb.arith.UInt64(0, 0), d = 0; d < a.length; d++) {
	        if ("0" > a[d] || "9" < a[d]) return null;
	        c.lo = parseInt(a[d], 10);
	        b = b.mul(10).add(c);
	    }
	    return b
	};
	jspb.arith.UInt64.prototype.clone = function () {
	    return new jspb.arith.UInt64(this.lo, this.hi)
	};
	jspb.arith.Int64 = function (a, b) {
	    this.lo = a;
	    this.hi = b;
	};
	jspb.arith.Int64.prototype.add = function (a) {
	    return new jspb.arith.Int64((this.lo + a.lo & 4294967295) >>> 0 >>> 0, ((this.hi + a.hi & 4294967295) >>> 0) + (4294967296 <= this.lo + a.lo ? 1 : 0) >>> 0)
	};
	jspb.arith.Int64.prototype.sub = function (a) {
	    return new jspb.arith.Int64((this.lo - a.lo & 4294967295) >>> 0 >>> 0, ((this.hi - a.hi & 4294967295) >>> 0) - (0 > this.lo - a.lo ? 1 : 0) >>> 0)
	};
	jspb.arith.Int64.prototype.clone = function () {
	    return new jspb.arith.Int64(this.lo, this.hi)
	};
	jspb.arith.Int64.prototype.toString = function () {
	    var a = 0 != (this.hi & 2147483648),
	        b = new jspb.arith.UInt64(this.lo, this.hi);
	    a && (b = (new jspb.arith.UInt64(0, 0)).sub(b));
	    return (a ? "-" : "") + b.toString()
	};
	jspb.arith.Int64.fromString = function (a) {
	    var b = 0 < a.length && "-" == a[0];
	    b && (a = a.substring(1));
	    a = jspb.arith.UInt64.fromString(a);
	    if (null === a) return null;
	    b && (a = (new jspb.arith.UInt64(0, 0)).sub(a));
	    return new jspb.arith.Int64(a.lo, a.hi)
	};
	jspb.BinaryConstants = {};
	var module$contents$jspb$BinaryConstants_FieldType = {
	        INVALID: -1,
	        DOUBLE: 1,
	        FLOAT: 2,
	        INT64: 3,
	        UINT64: 4,
	        INT32: 5,
	        FIXED64: 6,
	        FIXED32: 7,
	        BOOL: 8,
	        STRING: 9,
	        GROUP: 10,
	        MESSAGE: 11,
	        BYTES: 12,
	        UINT32: 13,
	        ENUM: 14,
	        SFIXED32: 15,
	        SFIXED64: 16,
	        SINT32: 17,
	        SINT64: 18
	    },
	    module$contents$jspb$BinaryConstants_WireType = {
	        INVALID: -1,
	        VARINT: 0,
	        FIXED64: 1,
	        DELIMITED: 2,
	        START_GROUP: 3,
	        END_GROUP: 4,
	        FIXED32: 5
	    };

	function module$contents$jspb$BinaryConstants_FieldTypeToWireType(a) {
	    switch (a) {
	        case module$contents$jspb$BinaryConstants_FieldType.INT32:
	        case module$contents$jspb$BinaryConstants_FieldType.INT64:
	        case module$contents$jspb$BinaryConstants_FieldType.UINT32:
	        case module$contents$jspb$BinaryConstants_FieldType.UINT64:
	        case module$contents$jspb$BinaryConstants_FieldType.SINT32:
	        case module$contents$jspb$BinaryConstants_FieldType.SINT64:
	        case module$contents$jspb$BinaryConstants_FieldType.BOOL:
	        case module$contents$jspb$BinaryConstants_FieldType.ENUM:
	            return module$contents$jspb$BinaryConstants_WireType.VARINT;
	        case module$contents$jspb$BinaryConstants_FieldType.DOUBLE:
	        case module$contents$jspb$BinaryConstants_FieldType.FIXED64:
	        case module$contents$jspb$BinaryConstants_FieldType.SFIXED64:
	            return module$contents$jspb$BinaryConstants_WireType.FIXED64;
	        case module$contents$jspb$BinaryConstants_FieldType.STRING:
	        case module$contents$jspb$BinaryConstants_FieldType.MESSAGE:
	        case module$contents$jspb$BinaryConstants_FieldType.BYTES:
	            return module$contents$jspb$BinaryConstants_WireType.DELIMITED;
	        case module$contents$jspb$BinaryConstants_FieldType.FLOAT:
	        case module$contents$jspb$BinaryConstants_FieldType.FIXED32:
	        case module$contents$jspb$BinaryConstants_FieldType.SFIXED32:
	            return module$contents$jspb$BinaryConstants_WireType.FIXED32;
	        default:
	            return module$contents$jspb$BinaryConstants_WireType.INVALID
	    }
	}
	jspb.BinaryConstants.FieldType = module$contents$jspb$BinaryConstants_FieldType;
	jspb.BinaryConstants.FieldTypeToWireType = module$contents$jspb$BinaryConstants_FieldTypeToWireType;
	jspb.BinaryConstants.FLOAT32_EPS = 1.401298464324817E-45;
	jspb.BinaryConstants.FLOAT32_MIN = 1.1754943508222875E-38;
	jspb.BinaryConstants.FLOAT32_MAX = 3.4028234663852886E38;
	jspb.BinaryConstants.FLOAT64_EPS = 4.9E-324;
	jspb.BinaryConstants.FLOAT64_MIN = 2.2250738585072014E-308;
	jspb.BinaryConstants.FLOAT64_MAX = 1.7976931348623157E308;
	jspb.BinaryConstants.INVALID_FIELD_NUMBER = -1;
	jspb.BinaryConstants.TWO_TO_20 = 1048576;
	jspb.BinaryConstants.TWO_TO_23 = 8388608;
	jspb.BinaryConstants.TWO_TO_31 = 2147483648;
	jspb.BinaryConstants.TWO_TO_32 = 4294967296;
	jspb.BinaryConstants.TWO_TO_52 = 4503599627370496;
	jspb.BinaryConstants.TWO_TO_63 = 0x7fffffffffffffff;
	jspb.BinaryConstants.TWO_TO_64 = 1.8446744073709552E19;
	jspb.BinaryConstants.WireType = module$contents$jspb$BinaryConstants_WireType;
	jspb.BinaryConstants.ZERO_HASH = "\x00\x00\x00\x00\x00\x00\x00\x00";
	jspb.ByteSource = void 0;
	goog.debug = {};

	function module$contents$goog$debug$Error_DebugError(a, b) {
	    if (Error.captureStackTrace) Error.captureStackTrace(this, module$contents$goog$debug$Error_DebugError);
	    else {
	        var c = Error().stack;
	        c && (this.stack = c);
	    }
	    a && (this.message = String(a));
	    b && (this.cause = b);
	    this.reportErrorToServer = !0;
	}
	goog.inherits(module$contents$goog$debug$Error_DebugError, Error);
	module$contents$goog$debug$Error_DebugError.prototype.name = "CustomError";
	goog.debug.Error = module$contents$goog$debug$Error_DebugError;
	goog.dom = {};
	goog.dom.NodeType = {
	    ELEMENT: 1,
	    ATTRIBUTE: 2,
	    TEXT: 3,
	    CDATA_SECTION: 4,
	    ENTITY_REFERENCE: 5,
	    ENTITY: 6,
	    PROCESSING_INSTRUCTION: 7,
	    COMMENT: 8,
	    DOCUMENT: 9,
	    DOCUMENT_TYPE: 10,
	    DOCUMENT_FRAGMENT: 11,
	    NOTATION: 12
	};
	goog.asserts = {};
	goog.asserts.ENABLE_ASSERTS = goog.DEBUG;
	goog.asserts.AssertionError = function (a, b) {
	    module$contents$goog$debug$Error_DebugError.call(this, goog.asserts.subs_(a, b));
	    this.messagePattern = a;
	};
	goog.inherits(goog.asserts.AssertionError, module$contents$goog$debug$Error_DebugError);
	goog.asserts.AssertionError.prototype.name = "AssertionError";
	goog.asserts.DEFAULT_ERROR_HANDLER = function (a) {
	    throw a;
	};
	goog.asserts.errorHandler_ = goog.asserts.DEFAULT_ERROR_HANDLER;
	goog.asserts.subs_ = function (a, b) {
	    a = a.split("%s");
	    for (var c = "", d = a.length - 1, e = 0; e < d; e++) c += a[e] + (e < b.length ? b[e] : "%s");
	    return c + a[d]
	};
	goog.asserts.doAssertFailure_ = function (a, b, c, d) {
	    var e = "Assertion failed";
	    if (c) {
	        e += ": " + c;
	        var f = d;
	    } else a && (e += ": " + a, f = b);
	    a = new goog.asserts.AssertionError("" + e, f || []);
	    goog.asserts.errorHandler_(a);
	};
	goog.asserts.setErrorHandler = function (a) {
	    goog.asserts.ENABLE_ASSERTS && (goog.asserts.errorHandler_ = a);
	};
	goog.asserts.assert = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && !a && goog.asserts.doAssertFailure_("", null, b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertExists = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && null == a && goog.asserts.doAssertFailure_("Expected to exist: %s.", [a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.fail = function (a, b) {
	    goog.asserts.ENABLE_ASSERTS && goog.asserts.errorHandler_(new goog.asserts.AssertionError("Failure" + (a ? ": " + a : ""), Array.prototype.slice.call(arguments, 1)));
	};
	goog.asserts.assertNumber = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && "number" !== typeof a && goog.asserts.doAssertFailure_("Expected number but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertString = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && "string" !== typeof a && goog.asserts.doAssertFailure_("Expected string but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertFunction = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && "function" !== typeof a && goog.asserts.doAssertFailure_("Expected function but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertObject = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && !goog.isObject(a) && goog.asserts.doAssertFailure_("Expected object but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertArray = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && !Array.isArray(a) && goog.asserts.doAssertFailure_("Expected array but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertBoolean = function (a, b, c) {
	    goog.asserts.ENABLE_ASSERTS && "boolean" !== typeof a && goog.asserts.doAssertFailure_("Expected boolean but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertElement = function (a, b, c) {
	    !goog.asserts.ENABLE_ASSERTS || goog.isObject(a) && a.nodeType == goog.dom.NodeType.ELEMENT || goog.asserts.doAssertFailure_("Expected Element but got %s: %s.", [goog.typeOf(a), a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.assertInstanceof = function (a, b, c, d) {
	    !goog.asserts.ENABLE_ASSERTS || a instanceof b || goog.asserts.doAssertFailure_("Expected instanceof %s but got %s.", [goog.asserts.getType_(b), goog.asserts.getType_(a)], c, Array.prototype.slice.call(arguments, 3));
	    return a
	};
	goog.asserts.assertFinite = function (a, b, c) {
	    !goog.asserts.ENABLE_ASSERTS || "number" == typeof a && isFinite(a) || goog.asserts.doAssertFailure_("Expected %s to be a finite number but it is not.", [a], b, Array.prototype.slice.call(arguments, 2));
	    return a
	};
	goog.asserts.getType_ = function (a) {
	    return a instanceof Function ? a.displayName || a.name || "unknown type name" : a instanceof Object ? a.constructor.displayName || a.constructor.name || Object.prototype.toString.call(a) : null === a ? "null" : typeof a
	};
	goog.array = {};
	goog.NATIVE_ARRAY_PROTOTYPES = goog.TRUSTED_SITE;
	var module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS = 2012 < goog.FEATURESET_YEAR;
	goog.array.ASSUME_NATIVE_FUNCTIONS = module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS;

	function module$contents$goog$array_peek(a) {
	    return a[a.length - 1]
	}
	goog.array.peek = module$contents$goog$array_peek;
	goog.array.last = module$contents$goog$array_peek;
	var module$contents$goog$array_indexOf = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.indexOf) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.indexOf.call(a, b, c)
	} : function (a, b, c) {
	    c = null == c ? 0 : 0 > c ? Math.max(0, a.length + c) : c;
	    if ("string" === typeof a) return "string" !== typeof b || 1 != b.length ? -1 : a.indexOf(b, c);
	    for (; c < a.length; c++)
	        if (c in a && a[c] === b) return c;
	    return -1
	};
	goog.array.indexOf = module$contents$goog$array_indexOf;
	var module$contents$goog$array_lastIndexOf = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.lastIndexOf) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.lastIndexOf.call(a, b, null == c ? a.length - 1 : c)
	} : function (a, b, c) {
	    c = null == c ? a.length - 1 : c;
	    0 > c && (c = Math.max(0, a.length + c));
	    if ("string" === typeof a) return "string" !== typeof b || 1 != b.length ? -1 : a.lastIndexOf(b, c);
	    for (; 0 <= c; c--)
	        if (c in a && a[c] === b) return c;
	    return -1
	};
	goog.array.lastIndexOf = module$contents$goog$array_lastIndexOf;
	var module$contents$goog$array_forEach = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.forEach) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    Array.prototype.forEach.call(a, b, c);
	} : function (a, b, c) {
	    for (var d = a.length, e = "string" === typeof a ? a.split("") : a, f = 0; f < d; f++) f in e && b.call(c, e[f], f, a);
	};
	goog.array.forEach = module$contents$goog$array_forEach;

	function module$contents$goog$array_forEachRight(a, b, c) {
	    var d = a.length,
	        e = "string" === typeof a ? a.split("") : a;
	    for (--d; 0 <= d; --d) d in e && b.call(c, e[d], d, a);
	}
	goog.array.forEachRight = module$contents$goog$array_forEachRight;
	goog.array.filter = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.filter) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.filter.call(a, b, c)
	} : function (a, b, c) {
	    for (var d = a.length, e = [], f = 0, g = "string" === typeof a ? a.split("") : a, h = 0; h < d; h++)
	        if (h in g) {
	            var k = g[h];
	            b.call(c, k, h, a) && (e[f++] = k);
	        } return e
	};
	var module$contents$goog$array_map = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.map) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.map.call(a, b, c)
	} : function (a, b, c) {
	    for (var d = a.length, e = Array(d), f = "string" === typeof a ? a.split("") : a, g = 0; g < d; g++) g in f && (e[g] = b.call(c, f[g], g, a));
	    return e
	};
	goog.array.map = module$contents$goog$array_map;
	goog.array.reduce = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.reduce) ? function (a, b, c, d) {
	    goog.asserts.assert(null != a.length);
	    d && (b = goog.bind(b, d));
	    return Array.prototype.reduce.call(a, b, c)
	} : function (a, b, c, d) {
	    var e = c;
	    module$contents$goog$array_forEach(a, function (f, g) {
	        e = b.call(d, e, f, g, a);
	    });
	    return e
	};
	goog.array.reduceRight = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.reduceRight) ? function (a, b, c, d) {
	    goog.asserts.assert(null != a.length);
	    goog.asserts.assert(null != b);
	    d && (b = goog.bind(b, d));
	    return Array.prototype.reduceRight.call(a, b, c)
	} : function (a, b, c, d) {
	    var e = c;
	    module$contents$goog$array_forEachRight(a, function (f, g) {
	        e = b.call(d, e, f, g, a);
	    });
	    return e
	};
	goog.array.some = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.some) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.some.call(a, b, c)
	} : function (a, b, c) {
	    for (var d = a.length, e = "string" === typeof a ? a.split("") : a, f = 0; f < d; f++)
	        if (f in e && b.call(c, e[f], f, a)) return !0;
	    return !1
	};
	goog.array.every = goog.NATIVE_ARRAY_PROTOTYPES && (module$contents$goog$array_ASSUME_NATIVE_FUNCTIONS || Array.prototype.every) ? function (a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.every.call(a, b, c)
	} : function (a, b, c) {
	    for (var d = a.length, e = "string" === typeof a ? a.split("") : a, f = 0; f < d; f++)
	        if (f in e && !b.call(c, e[f], f, a)) return !1;
	    return !0
	};
	goog.array.count = function (a, b, c) {
	    var d = 0;
	    module$contents$goog$array_forEach(a, function (e, f, g) {
	        b.call(c, e, f, g) && ++d;
	    }, c);
	    return d
	};

	function module$contents$goog$array_find(a, b, c) {
	    b = module$contents$goog$array_findIndex(a, b, c);
	    return 0 > b ? null : "string" === typeof a ? a.charAt(b) : a[b]
	}
	goog.array.find = module$contents$goog$array_find;

	function module$contents$goog$array_findIndex(a, b, c) {
	    for (var d = a.length, e = "string" === typeof a ? a.split("") : a, f = 0; f < d; f++)
	        if (f in e && b.call(c, e[f], f, a)) return f;
	    return -1
	}
	goog.array.findIndex = module$contents$goog$array_findIndex;
	goog.array.findRight = function (a, b, c) {
	    b = module$contents$goog$array_findIndexRight(a, b, c);
	    return 0 > b ? null : "string" === typeof a ? a.charAt(b) : a[b]
	};

	function module$contents$goog$array_findIndexRight(a, b, c) {
	    var d = a.length,
	        e = "string" === typeof a ? a.split("") : a;
	    for (--d; 0 <= d; d--)
	        if (d in e && b.call(c, e[d], d, a)) return d;
	    return -1
	}
	goog.array.findIndexRight = module$contents$goog$array_findIndexRight;

	function module$contents$goog$array_contains(a, b) {
	    return 0 <= module$contents$goog$array_indexOf(a, b)
	}
	goog.array.contains = module$contents$goog$array_contains;
	goog.array.isEmpty = function (a) {
	    return 0 == a.length
	};
	goog.array.clear = function (a) {
	    if (!Array.isArray(a))
	        for (var b = a.length - 1; 0 <= b; b--) delete a[b];
	    a.length = 0;
	};
	goog.array.insert = function (a, b) {
	    module$contents$goog$array_contains(a, b) || a.push(b);
	};

	function module$contents$goog$array_insertAt(a, b, c) {
	    module$contents$goog$array_splice(a, c, 0, b);
	}
	goog.array.insertAt = module$contents$goog$array_insertAt;
	goog.array.insertArrayAt = function (a, b, c) {
	    goog.partial(module$contents$goog$array_splice, a, c, 0).apply(null, b);
	};
	goog.array.insertBefore = function (a, b, c) {
	    var d;
	    2 == arguments.length || 0 > (d = module$contents$goog$array_indexOf(a, c)) ? a.push(b) : module$contents$goog$array_insertAt(a, b, d);
	};
	goog.array.remove = function (a, b) {
	    b = module$contents$goog$array_indexOf(a, b);
	    var c;
	    (c = 0 <= b) && module$contents$goog$array_removeAt(a, b);
	    return c
	};
	goog.array.removeLast = function (a, b) {
	    b = module$contents$goog$array_lastIndexOf(a, b);
	    return 0 <= b ? (module$contents$goog$array_removeAt(a, b), !0) : !1
	};

	function module$contents$goog$array_removeAt(a, b) {
	    goog.asserts.assert(null != a.length);
	    return 1 == Array.prototype.splice.call(a, b, 1).length
	}
	goog.array.removeAt = module$contents$goog$array_removeAt;
	goog.array.removeIf = function (a, b, c) {
	    b = module$contents$goog$array_findIndex(a, b, c);
	    return 0 <= b ? (module$contents$goog$array_removeAt(a, b), !0) : !1
	};
	goog.array.removeAllIf = function (a, b, c) {
	    var d = 0;
	    module$contents$goog$array_forEachRight(a, function (e, f) {
	        b.call(c, e, f, a) && module$contents$goog$array_removeAt(a, f) && d++;
	    });
	    return d
	};

	function module$contents$goog$array_concat(a) {
	    return Array.prototype.concat.apply([], arguments)
	}
	goog.array.concat = module$contents$goog$array_concat;
	goog.array.join = function (a) {
	    return Array.prototype.concat.apply([], arguments)
	};

	function module$contents$goog$array_toArray(a) {
	    var b = a.length;
	    if (0 < b) {
	        for (var c = Array(b), d = 0; d < b; d++) c[d] = a[d];
	        return c
	    }
	    return []
	}
	goog.array.toArray = module$contents$goog$array_toArray;
	goog.array.clone = module$contents$goog$array_toArray;
	goog.array.extend = function (a, b) {
	    for (var c = 1; c < arguments.length; c++) {
	        var d = arguments[c];
	        if (goog.isArrayLike(d)) {
	            var e = a.length || 0,
	                f = d.length || 0;
	            a.length = e + f;
	            for (var g = 0; g < f; g++) a[e + g] = d[g];
	        } else a.push(d);
	    }
	};

	function module$contents$goog$array_splice(a, b, c, d) {
	    goog.asserts.assert(null != a.length);
	    return Array.prototype.splice.apply(a, module$contents$goog$array_slice(arguments, 1))
	}
	goog.array.splice = module$contents$goog$array_splice;

	function module$contents$goog$array_slice(a, b, c) {
	    goog.asserts.assert(null != a.length);
	    return 2 >= arguments.length ? Array.prototype.slice.call(a, b) : Array.prototype.slice.call(a, b, c)
	}
	goog.array.slice = module$contents$goog$array_slice;
	goog.array.removeDuplicates = function (a, b, c) {
	    b = b || a;
	    var d = function (k) {
	        return goog.isObject(k) ? "o" + goog.getUid(k) : (typeof k).charAt(0) + k
	    };
	    c = c || d;
	    for (var e = d = 0, f = {}; e < a.length;) {
	        var g = a[e++],
	            h = c(g);
	        Object.prototype.hasOwnProperty.call(f, h) || (f[h] = !0, b[d++] = g);
	    }
	    b.length = d;
	};

	function module$contents$goog$array_binarySearch(a, b, c) {
	    return module$contents$goog$array_binarySearch_(a, c || module$contents$goog$array_defaultCompare, !1, b)
	}
	goog.array.binarySearch = module$contents$goog$array_binarySearch;
	goog.array.binarySelect = function (a, b, c) {
	    return module$contents$goog$array_binarySearch_(a, b, !0, void 0, c)
	};

	function module$contents$goog$array_binarySearch_(a, b, c, d, e) {
	    for (var f = 0, g = a.length, h; f < g;) {
	        var k = f + (g - f >>> 1);
	        var l = c ? b.call(e, a[k], k, a) : b(d, a[k]);
	        0 < l ? f = k + 1 : (g = k, h = !l);
	    }
	    return h ? f : -f - 1
	}

	function module$contents$goog$array_sort(a, b) {
	    a.sort(b || module$contents$goog$array_defaultCompare);
	}
	goog.array.sort = module$contents$goog$array_sort;
	goog.array.stableSort = function (a, b) {
	    for (var c = Array(a.length), d = 0; d < a.length; d++) c[d] = {
	        index: d,
	        value: a[d]
	    };
	    var e = b || module$contents$goog$array_defaultCompare;
	    module$contents$goog$array_sort(c, function (f, g) {
	        return e(f.value, g.value) || f.index - g.index
	    });
	    for (b = 0; b < a.length; b++) a[b] = c[b].value;
	};

	function module$contents$goog$array_sortByKey(a, b, c) {
	    var d = c || module$contents$goog$array_defaultCompare;
	    module$contents$goog$array_sort(a, function (e, f) {
	        return d(b(e), b(f))
	    });
	}
	goog.array.sortByKey = module$contents$goog$array_sortByKey;
	goog.array.sortObjectsByKey = function (a, b, c) {
	    module$contents$goog$array_sortByKey(a, function (d) {
	        return d[b]
	    }, c);
	};
	goog.array.isSorted = function (a, b, c) {
	    b = b || module$contents$goog$array_defaultCompare;
	    for (var d = 1; d < a.length; d++) {
	        var e = b(a[d - 1], a[d]);
	        if (0 < e || 0 == e && c) return !1
	    }
	    return !0
	};
	goog.array.equals = function (a, b, c) {
	    if (!goog.isArrayLike(a) || !goog.isArrayLike(b) || a.length != b.length) return !1;
	    var d = a.length;
	    c = c || module$contents$goog$array_defaultCompareEquality;
	    for (var e = 0; e < d; e++)
	        if (!c(a[e], b[e])) return !1;
	    return !0
	};
	goog.array.compare3 = function (a, b, c) {
	    c = c || module$contents$goog$array_defaultCompare;
	    for (var d = Math.min(a.length, b.length), e = 0; e < d; e++) {
	        var f = c(a[e], b[e]);
	        if (0 != f) return f
	    }
	    return module$contents$goog$array_defaultCompare(a.length, b.length)
	};

	function module$contents$goog$array_defaultCompare(a, b) {
	    return a > b ? 1 : a < b ? -1 : 0
	}
	goog.array.defaultCompare = module$contents$goog$array_defaultCompare;
	goog.array.inverseDefaultCompare = function (a, b) {
	    return -module$contents$goog$array_defaultCompare(a, b)
	};

	function module$contents$goog$array_defaultCompareEquality(a, b) {
	    return a === b
	}
	goog.array.defaultCompareEquality = module$contents$goog$array_defaultCompareEquality;
	goog.array.binaryInsert = function (a, b, c) {
	    c = module$contents$goog$array_binarySearch(a, b, c);
	    return 0 > c ? (module$contents$goog$array_insertAt(a, b, -(c + 1)), !0) : !1
	};
	goog.array.binaryRemove = function (a, b, c) {
	    b = module$contents$goog$array_binarySearch(a, b, c);
	    return 0 <= b ? module$contents$goog$array_removeAt(a, b) : !1
	};
	goog.array.bucket = function (a, b, c) {
	    for (var d = {}, e = 0; e < a.length; e++) {
	        var f = a[e],
	            g = b.call(c, f, e, a);
	        void 0 !== g && (d[g] || (d[g] = [])).push(f);
	    }
	    return d
	};
	goog.array.toObject = function (a, b, c) {
	    var d = {};
	    module$contents$goog$array_forEach(a, function (e, f) {
	        d[b.call(c, e, f, a)] = e;
	    });
	    return d
	};
	goog.array.range = function (a, b, c) {
	    var d = [],
	        e = 0,
	        f = a;
	    c = c || 1;
	    void 0 !== b && (e = a, f = b);
	    if (0 > c * (f - e)) return [];
	    if (0 < c)
	        for (a = e; a < f; a += c) d.push(a);
	    else
	        for (a = e; a > f; a += c) d.push(a);
	    return d
	};
	goog.array.repeat = function (a, b) {
	    for (var c = [], d = 0; d < b; d++) c[d] = a;
	    return c
	};

	function module$contents$goog$array_flatten(a) {
	    for (var b = [], c = 0; c < arguments.length; c++) {
	        var d = arguments[c];
	        if (Array.isArray(d))
	            for (var e = 0; e < d.length; e += 8192) {
	                var f = module$contents$goog$array_slice(d, e, e + 8192);
	                f = module$contents$goog$array_flatten.apply(null, f);
	                for (var g = 0; g < f.length; g++) b.push(f[g]);
	            } else b.push(d);
	    }
	    return b
	}
	goog.array.flatten = module$contents$goog$array_flatten;
	goog.array.rotate = function (a, b) {
	    goog.asserts.assert(null != a.length);
	    a.length && (b %= a.length, 0 < b ? Array.prototype.unshift.apply(a, a.splice(-b, b)) : 0 > b && Array.prototype.push.apply(a, a.splice(0, -b)));
	    return a
	};
	goog.array.moveItem = function (a, b, c) {
	    goog.asserts.assert(0 <= b && b < a.length);
	    goog.asserts.assert(0 <= c && c < a.length);
	    b = Array.prototype.splice.call(a, b, 1);
	    Array.prototype.splice.call(a, c, 0, b[0]);
	};
	goog.array.zip = function (a) {
	    if (!arguments.length) return [];
	    for (var b = [], c = arguments[0].length, d = 1; d < arguments.length; d++) arguments[d].length < c && (c = arguments[d].length);
	    for (d = 0; d < c; d++) {
	        for (var e = [], f = 0; f < arguments.length; f++) e.push(arguments[f][d]);
	        b.push(e);
	    }
	    return b
	};
	goog.array.shuffle = function (a, b) {
	    b = b || Math.random;
	    for (var c = a.length - 1; 0 < c; c--) {
	        var d = Math.floor(b() * (c + 1)),
	            e = a[c];
	        a[c] = a[d];
	        a[d] = e;
	    }
	};
	goog.array.copyByIndex = function (a, b) {
	    var c = [];
	    module$contents$goog$array_forEach(b, function (d) {
	        c.push(a[d]);
	    });
	    return c
	};
	goog.array.concatMap = function (a, b, c) {
	    return module$contents$goog$array_concat.apply([], module$contents$goog$array_map(a, b, c))
	};
	goog.crypt = {};
	goog.crypt.stringToByteArray = function (a) {
	    for (var b = [], c = 0, d = 0; d < a.length; d++) {
	        var e = a.charCodeAt(d);
	        255 < e && (b[c++] = e & 255, e >>= 8);
	        b[c++] = e;
	    }
	    return b
	};
	goog.crypt.byteArrayToString = function (a) {
	    if (8192 >= a.length) return String.fromCharCode.apply(null, a);
	    for (var b = "", c = 0; c < a.length; c += 8192) {
	        var d = module$contents$goog$array_slice(a, c, c + 8192);
	        b += String.fromCharCode.apply(null, d);
	    }
	    return b
	};
	goog.crypt.byteArrayToHex = function (a, b) {
	    return module$contents$goog$array_map(a, function (c) {
	        c = c.toString(16);
	        return 1 < c.length ? c : "0" + c
	    }).join(b || "")
	};
	goog.crypt.hexToByteArray = function (a) {
	    goog.asserts.assert(0 == a.length % 2, "Key string length must be multiple of 2");
	    for (var b = [], c = 0; c < a.length; c += 2) b.push(parseInt(a.substring(c, c + 2), 16));
	    return b
	};
	goog.crypt.stringToUtf8ByteArray = function (a) {
	    for (var b = [], c = 0, d = 0; d < a.length; d++) {
	        var e = a.charCodeAt(d);
	        128 > e ? b[c++] = e : (2048 > e ? b[c++] = e >> 6 | 192 : (55296 == (e & 64512) && d + 1 < a.length && 56320 == (a.charCodeAt(d + 1) & 64512) ? (e = 65536 + ((e & 1023) << 10) + (a.charCodeAt(++d) & 1023), b[c++] = e >> 18 | 240, b[c++] = e >> 12 & 63 | 128) : b[c++] = e >> 12 | 224, b[c++] = e >> 6 & 63 | 128), b[c++] = e & 63 | 128);
	    }
	    return b
	};
	goog.crypt.utf8ByteArrayToString = function (a) {
	    for (var b = [], c = 0, d = 0; c < a.length;) {
	        var e = a[c++];
	        if (128 > e) b[d++] = String.fromCharCode(e);
	        else if (191 < e && 224 > e) {
	            var f = a[c++];
	            b[d++] = String.fromCharCode((e & 31) << 6 | f & 63);
	        } else if (239 < e && 365 > e) {
	            f = a[c++];
	            var g = a[c++],
	                h = a[c++];
	            e = ((e & 7) << 18 | (f & 63) << 12 | (g & 63) << 6 | h & 63) - 65536;
	            b[d++] = String.fromCharCode(55296 + (e >> 10));
	            b[d++] = String.fromCharCode(56320 + (e & 1023));
	        } else f = a[c++], g = a[c++], b[d++] = String.fromCharCode((e & 15) << 12 | (f & 63) << 6 | g & 63);
	    }
	    return b.join("")
	};
	goog.crypt.xorByteArray = function (a, b) {
	    goog.asserts.assert(a.length == b.length, "XOR array lengths must match");
	    for (var c = [], d = 0; d < a.length; d++) c.push(a[d] ^ b[d]);
	    return c
	};
	goog.dom.asserts = {};
	goog.dom.asserts.assertIsLocation = function (a) {
	    if (goog.asserts.ENABLE_ASSERTS) {
	        var b = goog.dom.asserts.getWindow_(a);
	        b && (!a || !(a instanceof b.Location) && a instanceof b.Element) && goog.asserts.fail("Argument is not a Location (or a non-Element mock); got: %s", goog.dom.asserts.debugStringForType_(a));
	    }
	    return a
	};
	goog.dom.asserts.assertIsElementType_ = function (a, b) {
	    if (goog.asserts.ENABLE_ASSERTS) {
	        var c = goog.dom.asserts.getWindow_(a);
	        c && "undefined" != typeof c[b] && (a && (a instanceof c[b] || !(a instanceof c.Location || a instanceof c.Element)) || goog.asserts.fail("Argument is not a %s (or a non-Element, non-Location mock); got: %s", b, goog.dom.asserts.debugStringForType_(a)));
	    }
	    return a
	};
	goog.dom.asserts.assertIsHTMLAnchorElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLAnchorElement")
	};
	goog.dom.asserts.assertIsHTMLButtonElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLButtonElement")
	};
	goog.dom.asserts.assertIsHTMLLinkElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLLinkElement")
	};
	goog.dom.asserts.assertIsHTMLImageElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLImageElement")
	};
	goog.dom.asserts.assertIsHTMLAudioElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLAudioElement")
	};
	goog.dom.asserts.assertIsHTMLVideoElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLVideoElement")
	};
	goog.dom.asserts.assertIsHTMLInputElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLInputElement")
	};
	goog.dom.asserts.assertIsHTMLTextAreaElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLTextAreaElement")
	};
	goog.dom.asserts.assertIsHTMLCanvasElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLCanvasElement")
	};
	goog.dom.asserts.assertIsHTMLEmbedElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLEmbedElement")
	};
	goog.dom.asserts.assertIsHTMLFormElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLFormElement")
	};
	goog.dom.asserts.assertIsHTMLFrameElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLFrameElement")
	};
	goog.dom.asserts.assertIsHTMLIFrameElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLIFrameElement")
	};
	goog.dom.asserts.assertIsHTMLObjectElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLObjectElement")
	};
	goog.dom.asserts.assertIsHTMLScriptElement = function (a) {
	    return goog.dom.asserts.assertIsElementType_(a, "HTMLScriptElement")
	};
	goog.dom.asserts.debugStringForType_ = function (a) {
	    if (goog.isObject(a)) try {
	        return a.constructor.displayName || a.constructor.name || Object.prototype.toString.call(a)
	    } catch (b) {
	        return "<object could not be stringified>"
	    } else return void 0 === a ? "undefined" : null === a ? "null" : typeof a
	};
	goog.dom.asserts.getWindow_ = function (a) {
	    try {
	        var b = a && a.ownerDocument,
	            c = b && (b.defaultView || b.parentWindow);
	        c = c || goog.global;
	        if (c.Element && c.Location) return c
	    } catch (d) {}
	    return null
	};
	goog.functions = {};
	goog.functions.constant = function (a) {
	    return function () {
	        return a
	    }
	};
	goog.functions.FALSE = function () {
	    return !1
	};
	goog.functions.TRUE = function () {
	    return !0
	};
	goog.functions.NULL = function () {
	    return null
	};
	goog.functions.UNDEFINED = function () {};
	goog.functions.EMPTY = goog.functions.UNDEFINED;
	goog.functions.identity = function (a) {
	    return a
	};
	goog.functions.error = function (a) {
	    return function () {
	        throw Error(a);
	    }
	};
	goog.functions.fail = function (a) {
	    return function () {
	        throw a;
	    }
	};
	goog.functions.lock = function (a, b) {
	    b = b || 0;
	    return function () {
	        return a.apply(this, Array.prototype.slice.call(arguments, 0, b))
	    }
	};
	goog.functions.nth = function (a) {
	    return function () {
	        return arguments[a]
	    }
	};
	goog.functions.partialRight = function (a, b) {
	    var c = Array.prototype.slice.call(arguments, 1);
	    return function () {
	        var d = this;
	        d === goog.global && (d = void 0);
	        var e = Array.prototype.slice.call(arguments);
	        e.push.apply(e, c);
	        return a.apply(d, e)
	    }
	};
	goog.functions.withReturnValue = function (a, b) {
	    return goog.functions.sequence(a, goog.functions.constant(b))
	};
	goog.functions.equalTo = function (a, b) {
	    return function (c) {
	        return b ? a == c : a === c
	    }
	};
	goog.functions.compose = function (a, b) {
	    var c = arguments,
	        d = c.length;
	    return function () {
	        var e;
	        d && (e = c[d - 1].apply(this, arguments));
	        for (var f = d - 2; 0 <= f; f--) e = c[f].call(this, e);
	        return e
	    }
	};
	goog.functions.sequence = function (a) {
	    var b = arguments,
	        c = b.length;
	    return function () {
	        for (var d, e = 0; e < c; e++) d = b[e].apply(this, arguments);
	        return d
	    }
	};
	goog.functions.and = function (a) {
	    var b = arguments,
	        c = b.length;
	    return function () {
	        for (var d = 0; d < c; d++)
	            if (!b[d].apply(this, arguments)) return !1;
	        return !0
	    }
	};
	goog.functions.or = function (a) {
	    var b = arguments,
	        c = b.length;
	    return function () {
	        for (var d = 0; d < c; d++)
	            if (b[d].apply(this, arguments)) return !0;
	        return !1
	    }
	};
	goog.functions.not = function (a) {
	    return function () {
	        return !a.apply(this, arguments)
	    }
	};
	goog.functions.create = function (a, b) {
	    var c = function () {};
	    c.prototype = a.prototype;
	    c = new c;
	    a.apply(c, Array.prototype.slice.call(arguments, 1));
	    return c
	};
	goog.functions.CACHE_RETURN_VALUE = !0;
	goog.functions.cacheReturnValue = function (a) {
	    var b = !1,
	        c;
	    return function () {
	        if (!goog.functions.CACHE_RETURN_VALUE) return a();
	        b || (c = a(), b = !0);
	        return c
	    }
	};
	goog.functions.once = function (a) {
	    var b = a;
	    return function () {
	        if (b) {
	            var c = b;
	            b = null;
	            c();
	        }
	    }
	};
	goog.functions.debounce = function (a, b, c) {
	    var d = 0;
	    return function (e) {
	        goog.global.clearTimeout(d);
	        var f = arguments;
	        d = goog.global.setTimeout(function () {
	            a.apply(c, f);
	        }, b);
	    }
	};
	goog.functions.throttle = function (a, b, c) {
	    var d = 0,
	        e = !1,
	        f = [],
	        g = function () {
	            d = 0;
	            e && (e = !1, h());
	        },
	        h = function () {
	            d = goog.global.setTimeout(g, b);
	            var k = f;
	            f = [];
	            a.apply(c, k);
	        };
	    return function (k) {
	        f = arguments;
	        d ? e = !0 : h();
	    }
	};
	goog.functions.rateLimit = function (a, b, c) {
	    var d = 0,
	        e = function () {
	            d = 0;
	        };
	    return function (f) {
	        d || (d = goog.global.setTimeout(e, b), a.apply(c, arguments));
	    }
	};
	goog.functions.isFunction = function (a) {
	    return "function" === typeof a
	};
	goog.dom.HtmlElement = function () {};
	goog.dom.TagName = function () {};
	goog.dom.TagName.cast = function (a) {
	    return a
	};
	goog.dom.TagName.prototype.toString = function () {};
	goog.dom.TagName.A = "A";
	goog.dom.TagName.ABBR = "ABBR";
	goog.dom.TagName.ACRONYM = "ACRONYM";
	goog.dom.TagName.ADDRESS = "ADDRESS";
	goog.dom.TagName.APPLET = "APPLET";
	goog.dom.TagName.AREA = "AREA";
	goog.dom.TagName.ARTICLE = "ARTICLE";
	goog.dom.TagName.ASIDE = "ASIDE";
	goog.dom.TagName.AUDIO = "AUDIO";
	goog.dom.TagName.B = "B";
	goog.dom.TagName.BASE = "BASE";
	goog.dom.TagName.BASEFONT = "BASEFONT";
	goog.dom.TagName.BDI = "BDI";
	goog.dom.TagName.BDO = "BDO";
	goog.dom.TagName.BIG = "BIG";
	goog.dom.TagName.BLOCKQUOTE = "BLOCKQUOTE";
	goog.dom.TagName.BODY = "BODY";
	goog.dom.TagName.BR = "BR";
	goog.dom.TagName.BUTTON = "BUTTON";
	goog.dom.TagName.CANVAS = "CANVAS";
	goog.dom.TagName.CAPTION = "CAPTION";
	goog.dom.TagName.CENTER = "CENTER";
	goog.dom.TagName.CITE = "CITE";
	goog.dom.TagName.CODE = "CODE";
	goog.dom.TagName.COL = "COL";
	goog.dom.TagName.COLGROUP = "COLGROUP";
	goog.dom.TagName.COMMAND = "COMMAND";
	goog.dom.TagName.DATA = "DATA";
	goog.dom.TagName.DATALIST = "DATALIST";
	goog.dom.TagName.DD = "DD";
	goog.dom.TagName.DEL = "DEL";
	goog.dom.TagName.DETAILS = "DETAILS";
	goog.dom.TagName.DFN = "DFN";
	goog.dom.TagName.DIALOG = "DIALOG";
	goog.dom.TagName.DIR = "DIR";
	goog.dom.TagName.DIV = "DIV";
	goog.dom.TagName.DL = "DL";
	goog.dom.TagName.DT = "DT";
	goog.dom.TagName.EM = "EM";
	goog.dom.TagName.EMBED = "EMBED";
	goog.dom.TagName.FIELDSET = "FIELDSET";
	goog.dom.TagName.FIGCAPTION = "FIGCAPTION";
	goog.dom.TagName.FIGURE = "FIGURE";
	goog.dom.TagName.FONT = "FONT";
	goog.dom.TagName.FOOTER = "FOOTER";
	goog.dom.TagName.FORM = "FORM";
	goog.dom.TagName.FRAME = "FRAME";
	goog.dom.TagName.FRAMESET = "FRAMESET";
	goog.dom.TagName.H1 = "H1";
	goog.dom.TagName.H2 = "H2";
	goog.dom.TagName.H3 = "H3";
	goog.dom.TagName.H4 = "H4";
	goog.dom.TagName.H5 = "H5";
	goog.dom.TagName.H6 = "H6";
	goog.dom.TagName.HEAD = "HEAD";
	goog.dom.TagName.HEADER = "HEADER";
	goog.dom.TagName.HGROUP = "HGROUP";
	goog.dom.TagName.HR = "HR";
	goog.dom.TagName.HTML = "HTML";
	goog.dom.TagName.I = "I";
	goog.dom.TagName.IFRAME = "IFRAME";
	goog.dom.TagName.IMG = "IMG";
	goog.dom.TagName.INPUT = "INPUT";
	goog.dom.TagName.INS = "INS";
	goog.dom.TagName.ISINDEX = "ISINDEX";
	goog.dom.TagName.KBD = "KBD";
	goog.dom.TagName.KEYGEN = "KEYGEN";
	goog.dom.TagName.LABEL = "LABEL";
	goog.dom.TagName.LEGEND = "LEGEND";
	goog.dom.TagName.LI = "LI";
	goog.dom.TagName.LINK = "LINK";
	goog.dom.TagName.MAIN = "MAIN";
	goog.dom.TagName.MAP = "MAP";
	goog.dom.TagName.MARK = "MARK";
	goog.dom.TagName.MATH = "MATH";
	goog.dom.TagName.MENU = "MENU";
	goog.dom.TagName.MENUITEM = "MENUITEM";
	goog.dom.TagName.META = "META";
	goog.dom.TagName.METER = "METER";
	goog.dom.TagName.NAV = "NAV";
	goog.dom.TagName.NOFRAMES = "NOFRAMES";
	goog.dom.TagName.NOSCRIPT = "NOSCRIPT";
	goog.dom.TagName.OBJECT = "OBJECT";
	goog.dom.TagName.OL = "OL";
	goog.dom.TagName.OPTGROUP = "OPTGROUP";
	goog.dom.TagName.OPTION = "OPTION";
	goog.dom.TagName.OUTPUT = "OUTPUT";
	goog.dom.TagName.P = "P";
	goog.dom.TagName.PARAM = "PARAM";
	goog.dom.TagName.PICTURE = "PICTURE";
	goog.dom.TagName.PRE = "PRE";
	goog.dom.TagName.PROGRESS = "PROGRESS";
	goog.dom.TagName.Q = "Q";
	goog.dom.TagName.RP = "RP";
	goog.dom.TagName.RT = "RT";
	goog.dom.TagName.RTC = "RTC";
	goog.dom.TagName.RUBY = "RUBY";
	goog.dom.TagName.S = "S";
	goog.dom.TagName.SAMP = "SAMP";
	goog.dom.TagName.SCRIPT = "SCRIPT";
	goog.dom.TagName.SECTION = "SECTION";
	goog.dom.TagName.SELECT = "SELECT";
	goog.dom.TagName.SMALL = "SMALL";
	goog.dom.TagName.SOURCE = "SOURCE";
	goog.dom.TagName.SPAN = "SPAN";
	goog.dom.TagName.STRIKE = "STRIKE";
	goog.dom.TagName.STRONG = "STRONG";
	goog.dom.TagName.STYLE = "STYLE";
	goog.dom.TagName.SUB = "SUB";
	goog.dom.TagName.SUMMARY = "SUMMARY";
	goog.dom.TagName.SUP = "SUP";
	goog.dom.TagName.SVG = "SVG";
	goog.dom.TagName.TABLE = "TABLE";
	goog.dom.TagName.TBODY = "TBODY";
	goog.dom.TagName.TD = "TD";
	goog.dom.TagName.TEMPLATE = "TEMPLATE";
	goog.dom.TagName.TEXTAREA = "TEXTAREA";
	goog.dom.TagName.TFOOT = "TFOOT";
	goog.dom.TagName.TH = "TH";
	goog.dom.TagName.THEAD = "THEAD";
	goog.dom.TagName.TIME = "TIME";
	goog.dom.TagName.TITLE = "TITLE";
	goog.dom.TagName.TR = "TR";
	goog.dom.TagName.TRACK = "TRACK";
	goog.dom.TagName.TT = "TT";
	goog.dom.TagName.U = "U";
	goog.dom.TagName.UL = "UL";
	goog.dom.TagName.VAR = "VAR";
	goog.dom.TagName.VIDEO = "VIDEO";
	goog.dom.TagName.WBR = "WBR";
	goog.object = {};
	goog.object.forEach = function (a, b, c) {
	    for (var d in a) b.call(c, a[d], d, a);
	};
	goog.object.filter = function (a, b, c) {
	    var d = {},
	        e;
	    for (e in a) b.call(c, a[e], e, a) && (d[e] = a[e]);
	    return d
	};
	goog.object.map = function (a, b, c) {
	    var d = {},
	        e;
	    for (e in a) d[e] = b.call(c, a[e], e, a);
	    return d
	};
	goog.object.some = function (a, b, c) {
	    for (var d in a)
	        if (b.call(c, a[d], d, a)) return !0;
	    return !1
	};
	goog.object.every = function (a, b, c) {
	    for (var d in a)
	        if (!b.call(c, a[d], d, a)) return !1;
	    return !0
	};
	goog.object.getCount = function (a) {
	    var b = 0,
	        c;
	    for (c in a) b++;
	    return b
	};
	goog.object.getAnyKey = function (a) {
	    for (var b in a) return b
	};
	goog.object.getAnyValue = function (a) {
	    for (var b in a) return a[b]
	};
	goog.object.contains = function (a, b) {
	    return goog.object.containsValue(a, b)
	};
	goog.object.getValues = function (a) {
	    var b = [],
	        c = 0,
	        d;
	    for (d in a) b[c++] = a[d];
	    return b
	};
	goog.object.getKeys = function (a) {
	    var b = [],
	        c = 0,
	        d;
	    for (d in a) b[c++] = d;
	    return b
	};
	goog.object.getValueByKeys = function (a, b) {
	    var c = goog.isArrayLike(b),
	        d = c ? b : arguments;
	    for (c = c ? 0 : 1; c < d.length; c++) {
	        if (null == a) return;
	        a = a[d[c]];
	    }
	    return a
	};
	goog.object.containsKey = function (a, b) {
	    return null !== a && b in a
	};
	goog.object.containsValue = function (a, b) {
	    for (var c in a)
	        if (a[c] == b) return !0;
	    return !1
	};
	goog.object.findKey = function (a, b, c) {
	    for (var d in a)
	        if (b.call(c, a[d], d, a)) return d
	};
	goog.object.findValue = function (a, b, c) {
	    return (b = goog.object.findKey(a, b, c)) && a[b]
	};
	goog.object.isEmpty = function (a) {
	    for (var b in a) return !1;
	    return !0
	};
	goog.object.clear = function (a) {
	    for (var b in a) delete a[b];
	};
	goog.object.remove = function (a, b) {
	    var c;
	    (c = b in a) && delete a[b];
	    return c
	};
	goog.object.add = function (a, b, c) {
	    if (null !== a && b in a) throw Error('The object already contains the key "' + b + '"');
	    goog.object.set(a, b, c);
	};
	goog.object.get = function (a, b, c) {
	    return null !== a && b in a ? a[b] : c
	};
	goog.object.set = function (a, b, c) {
	    a[b] = c;
	};
	goog.object.setIfUndefined = function (a, b, c) {
	    return b in a ? a[b] : a[b] = c
	};
	goog.object.setWithReturnValueIfNotSet = function (a, b, c) {
	    if (b in a) return a[b];
	    c = c();
	    return a[b] = c
	};
	goog.object.equals = function (a, b) {
	    for (var c in a)
	        if (!(c in b) || a[c] !== b[c]) return !1;
	    for (var d in b)
	        if (!(d in a)) return !1;
	    return !0
	};
	goog.object.clone = function (a) {
	    var b = {},
	        c;
	    for (c in a) b[c] = a[c];
	    return b
	};
	goog.object.unsafeClone = function (a) {
	    if (!a || "object" !== typeof a) return a;
	    if ("function" === typeof a.clone) return a.clone();
	    var b = Array.isArray(a) ? [] : "function" !== typeof ArrayBuffer || "function" !== typeof ArrayBuffer.isView || !ArrayBuffer.isView(a) || a instanceof DataView ? {} : new a.constructor(a.length),
	        c;
	    for (c in a) b[c] = goog.object.unsafeClone(a[c]);
	    return b
	};
	goog.object.transpose = function (a) {
	    var b = {},
	        c;
	    for (c in a) b[a[c]] = c;
	    return b
	};
	goog.object.PROTOTYPE_FIELDS_ = "constructor hasOwnProperty isPrototypeOf propertyIsEnumerable toLocaleString toString valueOf".split(" ");
	goog.object.extend = function (a, b) {
	    for (var c, d, e = 1; e < arguments.length; e++) {
	        d = arguments[e];
	        for (c in d) a[c] = d[c];
	        for (var f = 0; f < goog.object.PROTOTYPE_FIELDS_.length; f++) c = goog.object.PROTOTYPE_FIELDS_[f], Object.prototype.hasOwnProperty.call(d, c) && (a[c] = d[c]);
	    }
	};
	goog.object.create = function (a) {
	    var b = arguments.length;
	    if (1 == b && Array.isArray(arguments[0])) return goog.object.create.apply(null, arguments[0]);
	    if (b % 2) throw Error("Uneven number of arguments");
	    for (var c = {}, d = 0; d < b; d += 2) c[arguments[d]] = arguments[d + 1];
	    return c
	};
	goog.object.createSet = function (a) {
	    var b = arguments.length;
	    if (1 == b && Array.isArray(arguments[0])) return goog.object.createSet.apply(null, arguments[0]);
	    for (var c = {}, d = 0; d < b; d++) c[arguments[d]] = !0;
	    return c
	};
	goog.object.createImmutableView = function (a) {
	    var b = a;
	    Object.isFrozen && !Object.isFrozen(a) && (b = Object.create(a), Object.freeze(b));
	    return b
	};
	goog.object.isImmutableView = function (a) {
	    return !!Object.isFrozen && Object.isFrozen(a)
	};
	goog.object.getAllPropertyNames = function (a, b, c) {
	    if (!a) return [];
	    if (!Object.getOwnPropertyNames || !Object.getPrototypeOf) return goog.object.getKeys(a);
	    for (var d = {}; a && (a !== Object.prototype || b) && (a !== Function.prototype || c);) {
	        for (var e = Object.getOwnPropertyNames(a), f = 0; f < e.length; f++) d[e[f]] = !0;
	        a = Object.getPrototypeOf(a);
	    }
	    return goog.object.getKeys(d)
	};
	goog.object.getSuperClass = function (a) {
	    return (a = Object.getPrototypeOf(a.prototype)) && a.constructor
	};
	goog.dom.tags = {};
	goog.dom.tags.VOID_TAGS_ = {
	    area: !0,
	    base: !0,
	    br: !0,
	    col: !0,
	    command: !0,
	    embed: !0,
	    hr: !0,
	    img: !0,
	    input: !0,
	    keygen: !0,
	    link: !0,
	    meta: !0,
	    param: !0,
	    source: !0,
	    track: !0,
	    wbr: !0
	};
	goog.dom.tags.isVoidTag = function (a) {
	    return !0 === goog.dom.tags.VOID_TAGS_[a]
	};
	goog.html = {};
	goog.html.trustedtypes = {};
	goog.html.trustedtypes.getPolicyPrivateDoNotAccessOrElse = function () {
	    if (!goog.TRUSTED_TYPES_POLICY_NAME) return null;
	    void 0 === goog.html.trustedtypes.cachedPolicy_ && (goog.html.trustedtypes.cachedPolicy_ = goog.createTrustedTypesPolicy(goog.TRUSTED_TYPES_POLICY_NAME + "#html"));
	    return goog.html.trustedtypes.cachedPolicy_
	};
	goog.string = {};
	goog.string.TypedString = function () {};
	goog.string.Const = function (a, b) {
	    this.stringConstValueWithSecurityContract__googStringSecurityPrivate_ = a === goog.string.Const.GOOG_STRING_CONSTRUCTOR_TOKEN_PRIVATE_ && b || "";
	    this.STRING_CONST_TYPE_MARKER__GOOG_STRING_SECURITY_PRIVATE_ = goog.string.Const.TYPE_MARKER_;
	};
	goog.string.Const.prototype.implementsGoogStringTypedString = !0;
	goog.string.Const.prototype.getTypedStringValue = function () {
	    return this.stringConstValueWithSecurityContract__googStringSecurityPrivate_
	};
	goog.DEBUG && (goog.string.Const.prototype.toString = function () {
	    return "Const{" + this.stringConstValueWithSecurityContract__googStringSecurityPrivate_ + "}"
	});
	goog.string.Const.unwrap = function (a) {
	    if (a instanceof goog.string.Const && a.constructor === goog.string.Const && a.STRING_CONST_TYPE_MARKER__GOOG_STRING_SECURITY_PRIVATE_ === goog.string.Const.TYPE_MARKER_) return a.stringConstValueWithSecurityContract__googStringSecurityPrivate_;
	    goog.asserts.fail("expected object of type Const, got '" + a + "'");
	    return "type_error:Const"
	};
	goog.string.Const.from = function (a) {
	    return new goog.string.Const(goog.string.Const.GOOG_STRING_CONSTRUCTOR_TOKEN_PRIVATE_, a)
	};
	goog.string.Const.TYPE_MARKER_ = {};
	goog.string.Const.GOOG_STRING_CONSTRUCTOR_TOKEN_PRIVATE_ = {};
	goog.string.Const.EMPTY = goog.string.Const.from("");
	var module$contents$goog$html$SafeScript_CONSTRUCTOR_TOKEN_PRIVATE = {},
	    module$contents$goog$html$SafeScript_SafeScript = function (a, b) {
	        this.privateDoNotAccessOrElseSafeScriptWrappedValue_ = b === module$contents$goog$html$SafeScript_CONSTRUCTOR_TOKEN_PRIVATE ? a : "";
	        this.implementsGoogStringTypedString = !0;
	    };
	module$contents$goog$html$SafeScript_SafeScript.fromConstant = function (a) {
	    a = goog.string.Const.unwrap(a);
	    return 0 === a.length ? module$contents$goog$html$SafeScript_SafeScript.EMPTY : module$contents$goog$html$SafeScript_SafeScript.createSafeScriptSecurityPrivateDoNotAccessOrElse(a)
	};
	module$contents$goog$html$SafeScript_SafeScript.fromJson = function (a) {
	    return module$contents$goog$html$SafeScript_SafeScript.createSafeScriptSecurityPrivateDoNotAccessOrElse(module$contents$goog$html$SafeScript_SafeScript.stringify_(a))
	};
	module$contents$goog$html$SafeScript_SafeScript.prototype.getTypedStringValue = function () {
	    return this.privateDoNotAccessOrElseSafeScriptWrappedValue_.toString()
	};
	module$contents$goog$html$SafeScript_SafeScript.unwrap = function (a) {
	    return module$contents$goog$html$SafeScript_SafeScript.unwrapTrustedScript(a).toString()
	};
	module$contents$goog$html$SafeScript_SafeScript.unwrapTrustedScript = function (a) {
	    if (a instanceof module$contents$goog$html$SafeScript_SafeScript && a.constructor === module$contents$goog$html$SafeScript_SafeScript) return a.privateDoNotAccessOrElseSafeScriptWrappedValue_;
	    (0, goog.asserts.fail)("expected object of type SafeScript, got '" + a + "' of type " + goog.typeOf(a));
	    return "type_error:SafeScript"
	};
	module$contents$goog$html$SafeScript_SafeScript.stringify_ = function (a) {
	    return JSON.stringify(a).replace(/</g, "\\x3c")
	};
	module$contents$goog$html$SafeScript_SafeScript.createSafeScriptSecurityPrivateDoNotAccessOrElse = function (a) {
	    var b = goog.html.trustedtypes.getPolicyPrivateDoNotAccessOrElse();
	    a = b ? b.createScript(a) : a;
	    return new module$contents$goog$html$SafeScript_SafeScript(a, module$contents$goog$html$SafeScript_CONSTRUCTOR_TOKEN_PRIVATE)
	};
	module$contents$goog$html$SafeScript_SafeScript.prototype.toString = function () {
	    return this.privateDoNotAccessOrElseSafeScriptWrappedValue_.toString()
	};
	module$contents$goog$html$SafeScript_SafeScript.EMPTY = module$contents$goog$html$SafeScript_SafeScript.createSafeScriptSecurityPrivateDoNotAccessOrElse("");
	goog.html.SafeScript = module$contents$goog$html$SafeScript_SafeScript;
	goog.fs = {};
	goog.fs.url = {};
	goog.fs.url.createObjectUrl = function (a) {
	    return goog.fs.url.getUrlObject_().createObjectURL(a)
	};
	goog.fs.url.revokeObjectUrl = function (a) {
	    goog.fs.url.getUrlObject_().revokeObjectURL(a);
	};
	goog.fs.url.UrlObject_ = function () {};
	goog.fs.url.UrlObject_.prototype.createObjectURL = function () {};
	goog.fs.url.UrlObject_.prototype.revokeObjectURL = function () {};
	goog.fs.url.getUrlObject_ = function () {
	    var a = goog.fs.url.findUrlObject_();
	    if (null != a) return a;
	    throw Error("This browser doesn't seem to support blob URLs");
	};
	goog.fs.url.findUrlObject_ = function () {
	    return void 0 !== goog.global.URL && void 0 !== goog.global.URL.createObjectURL ? goog.global.URL : void 0 !== goog.global.createObjectURL ? goog.global : null
	};
	goog.fs.url.browserSupportsObjectUrls = function () {
	    return null != goog.fs.url.findUrlObject_()
	};
	goog.fs.blob = {};
	goog.fs.blob.getBlob = function (a) {
	    var b = goog.global.BlobBuilder || goog.global.WebKitBlobBuilder;
	    if (void 0 !== b) {
	        b = new b;
	        for (var c = 0; c < arguments.length; c++) b.append(arguments[c]);
	        return b.getBlob()
	    }
	    return goog.fs.blob.getBlobWithProperties(module$contents$goog$array_toArray(arguments))
	};
	goog.fs.blob.getBlobWithProperties = function (a, b, c) {
	    var d = goog.global.BlobBuilder || goog.global.WebKitBlobBuilder;
	    if (void 0 !== d) {
	        d = new d;
	        for (var e = 0; e < a.length; e++) d.append(a[e], c);
	        return d.getBlob(b)
	    }
	    if (void 0 !== goog.global.Blob) return d = {}, b && (d.type = b), c && (d.endings = c), new Blob(a, d);
	    throw Error("This browser doesn't seem to support creating Blobs");
	};
	goog.i18n = {};
	goog.i18n.bidi = {};
	goog.i18n.bidi.FORCE_RTL = !1;
	goog.i18n.bidi.IS_RTL = goog.i18n.bidi.FORCE_RTL || ("ar" == goog.LOCALE.substring(0, 2).toLowerCase() || "fa" == goog.LOCALE.substring(0, 2).toLowerCase() || "he" == goog.LOCALE.substring(0, 2).toLowerCase() || "iw" == goog.LOCALE.substring(0, 2).toLowerCase() || "ps" == goog.LOCALE.substring(0, 2).toLowerCase() || "sd" == goog.LOCALE.substring(0, 2).toLowerCase() || "ug" == goog.LOCALE.substring(0, 2).toLowerCase() || "ur" == goog.LOCALE.substring(0, 2).toLowerCase() || "yi" == goog.LOCALE.substring(0, 2).toLowerCase()) && (2 == goog.LOCALE.length ||
	    "-" == goog.LOCALE.substring(2, 3) || "_" == goog.LOCALE.substring(2, 3)) || 3 <= goog.LOCALE.length && "ckb" == goog.LOCALE.substring(0, 3).toLowerCase() && (3 == goog.LOCALE.length || "-" == goog.LOCALE.substring(3, 4) || "_" == goog.LOCALE.substring(3, 4)) || 7 <= goog.LOCALE.length && ("-" == goog.LOCALE.substring(2, 3) || "_" == goog.LOCALE.substring(2, 3)) && ("adlm" == goog.LOCALE.substring(3, 7).toLowerCase() || "arab" == goog.LOCALE.substring(3, 7).toLowerCase() || "hebr" == goog.LOCALE.substring(3, 7).toLowerCase() || "nkoo" == goog.LOCALE.substring(3,
	    7).toLowerCase() || "rohg" == goog.LOCALE.substring(3, 7).toLowerCase() || "thaa" == goog.LOCALE.substring(3, 7).toLowerCase()) || 8 <= goog.LOCALE.length && ("-" == goog.LOCALE.substring(3, 4) || "_" == goog.LOCALE.substring(3, 4)) && ("adlm" == goog.LOCALE.substring(4, 8).toLowerCase() || "arab" == goog.LOCALE.substring(4, 8).toLowerCase() || "hebr" == goog.LOCALE.substring(4, 8).toLowerCase() || "nkoo" == goog.LOCALE.substring(4, 8).toLowerCase() || "rohg" == goog.LOCALE.substring(4, 8).toLowerCase() || "thaa" == goog.LOCALE.substring(4, 8).toLowerCase());
	goog.i18n.bidi.Format = {
	    LRE: "\u202a",
	    RLE: "\u202b",
	    PDF: "\u202c",
	    LRM: "\u200e",
	    RLM: "\u200f"
	};
	goog.i18n.bidi.Dir = {
	    LTR: 1,
	    RTL: -1,
	    NEUTRAL: 0
	};
	goog.i18n.bidi.RIGHT = "right";
	goog.i18n.bidi.LEFT = "left";
	goog.i18n.bidi.I18N_RIGHT = goog.i18n.bidi.IS_RTL ? goog.i18n.bidi.LEFT : goog.i18n.bidi.RIGHT;
	goog.i18n.bidi.I18N_LEFT = goog.i18n.bidi.IS_RTL ? goog.i18n.bidi.RIGHT : goog.i18n.bidi.LEFT;
	goog.i18n.bidi.toDir = function (a, b) {
	    return "number" == typeof a ? 0 < a ? goog.i18n.bidi.Dir.LTR : 0 > a ? goog.i18n.bidi.Dir.RTL : b ? null : goog.i18n.bidi.Dir.NEUTRAL : null == a ? null : a ? goog.i18n.bidi.Dir.RTL : goog.i18n.bidi.Dir.LTR
	};
	goog.i18n.bidi.ltrChars_ = "A-Za-z\u00c0-\u00d6\u00d8-\u00f6\u00f8-\u02b8\u0300-\u0590\u0900-\u1fff\u200e\u2c00-\ud801\ud804-\ud839\ud83c-\udbff\uf900-\ufb1c\ufe00-\ufe6f\ufefd-\uffff";
	goog.i18n.bidi.rtlChars_ = "\u0591-\u06ef\u06fa-\u08ff\u200f\ud802-\ud803\ud83a-\ud83b\ufb1d-\ufdff\ufe70-\ufefc";
	goog.i18n.bidi.htmlSkipReg_ = /<[^>]*>|&[^;]+;/g;
	goog.i18n.bidi.stripHtmlIfNeeded_ = function (a, b) {
	    return b ? a.replace(goog.i18n.bidi.htmlSkipReg_, "") : a
	};
	goog.i18n.bidi.rtlCharReg_ = new RegExp("[" + goog.i18n.bidi.rtlChars_ + "]");
	goog.i18n.bidi.ltrCharReg_ = new RegExp("[" + goog.i18n.bidi.ltrChars_ + "]");
	goog.i18n.bidi.hasAnyRtl = function (a, b) {
	    return goog.i18n.bidi.rtlCharReg_.test(goog.i18n.bidi.stripHtmlIfNeeded_(a, b))
	};
	goog.i18n.bidi.hasRtlChar = goog.i18n.bidi.hasAnyRtl;
	goog.i18n.bidi.hasAnyLtr = function (a, b) {
	    return goog.i18n.bidi.ltrCharReg_.test(goog.i18n.bidi.stripHtmlIfNeeded_(a, b))
	};
	goog.i18n.bidi.ltrRe_ = new RegExp("^[" + goog.i18n.bidi.ltrChars_ + "]");
	goog.i18n.bidi.rtlRe_ = new RegExp("^[" + goog.i18n.bidi.rtlChars_ + "]");
	goog.i18n.bidi.isRtlChar = function (a) {
	    return goog.i18n.bidi.rtlRe_.test(a)
	};
	goog.i18n.bidi.isLtrChar = function (a) {
	    return goog.i18n.bidi.ltrRe_.test(a)
	};
	goog.i18n.bidi.isNeutralChar = function (a) {
	    return !goog.i18n.bidi.isLtrChar(a) && !goog.i18n.bidi.isRtlChar(a)
	};
	goog.i18n.bidi.ltrDirCheckRe_ = new RegExp("^[^" + goog.i18n.bidi.rtlChars_ + "]*[" + goog.i18n.bidi.ltrChars_ + "]");
	goog.i18n.bidi.rtlDirCheckRe_ = new RegExp("^[^" + goog.i18n.bidi.ltrChars_ + "]*[" + goog.i18n.bidi.rtlChars_ + "]");
	goog.i18n.bidi.startsWithRtl = function (a, b) {
	    return goog.i18n.bidi.rtlDirCheckRe_.test(goog.i18n.bidi.stripHtmlIfNeeded_(a, b))
	};
	goog.i18n.bidi.isRtlText = goog.i18n.bidi.startsWithRtl;
	goog.i18n.bidi.startsWithLtr = function (a, b) {
	    return goog.i18n.bidi.ltrDirCheckRe_.test(goog.i18n.bidi.stripHtmlIfNeeded_(a, b))
	};
	goog.i18n.bidi.isLtrText = goog.i18n.bidi.startsWithLtr;
	goog.i18n.bidi.isRequiredLtrRe_ = /^http:\/\/.*/;
	goog.i18n.bidi.isNeutralText = function (a, b) {
	    a = goog.i18n.bidi.stripHtmlIfNeeded_(a, b);
	    return goog.i18n.bidi.isRequiredLtrRe_.test(a) || !goog.i18n.bidi.hasAnyLtr(a) && !goog.i18n.bidi.hasAnyRtl(a)
	};
	goog.i18n.bidi.ltrExitDirCheckRe_ = new RegExp("[" + goog.i18n.bidi.ltrChars_ + "][^" + goog.i18n.bidi.rtlChars_ + "]*$");
	goog.i18n.bidi.rtlExitDirCheckRe_ = new RegExp("[" + goog.i18n.bidi.rtlChars_ + "][^" + goog.i18n.bidi.ltrChars_ + "]*$");
	goog.i18n.bidi.endsWithLtr = function (a, b) {
	    return goog.i18n.bidi.ltrExitDirCheckRe_.test(goog.i18n.bidi.stripHtmlIfNeeded_(a, b))
	};
	goog.i18n.bidi.isLtrExitText = goog.i18n.bidi.endsWithLtr;
	goog.i18n.bidi.endsWithRtl = function (a, b) {
	    return goog.i18n.bidi.rtlExitDirCheckRe_.test(goog.i18n.bidi.stripHtmlIfNeeded_(a, b))
	};
	goog.i18n.bidi.isRtlExitText = goog.i18n.bidi.endsWithRtl;
	goog.i18n.bidi.rtlLocalesRe_ = /^(ar|ckb|dv|he|iw|fa|nqo|ps|sd|ug|ur|yi|.*[-_](Adlm|Arab|Hebr|Nkoo|Rohg|Thaa))(?!.*[-_](Latn|Cyrl)($|-|_))($|-|_)/i;
	goog.i18n.bidi.isRtlLanguage = function (a) {
	    return goog.i18n.bidi.rtlLocalesRe_.test(a)
	};
	goog.i18n.bidi.bracketGuardTextRe_ = /(\(.*?\)+)|(\[.*?\]+)|(\{.*?\}+)|(<.*?>+)/g;
	goog.i18n.bidi.guardBracketInText = function (a, b) {
	    b = (void 0 === b ? goog.i18n.bidi.hasAnyRtl(a) : b) ? goog.i18n.bidi.Format.RLM : goog.i18n.bidi.Format.LRM;
	    return a.replace(goog.i18n.bidi.bracketGuardTextRe_, b + "$&" + b)
	};
	goog.i18n.bidi.enforceRtlInHtml = function (a) {
	    return "<" == a.charAt(0) ? a.replace(/<\w+/, "$& dir=rtl") : "\n<span dir=rtl>" + a + "</span>"
	};
	goog.i18n.bidi.enforceRtlInText = function (a) {
	    return goog.i18n.bidi.Format.RLE + a + goog.i18n.bidi.Format.PDF
	};
	goog.i18n.bidi.enforceLtrInHtml = function (a) {
	    return "<" == a.charAt(0) ? a.replace(/<\w+/, "$& dir=ltr") : "\n<span dir=ltr>" + a + "</span>"
	};
	goog.i18n.bidi.enforceLtrInText = function (a) {
	    return goog.i18n.bidi.Format.LRE + a + goog.i18n.bidi.Format.PDF
	};
	goog.i18n.bidi.dimensionsRe_ = /:\s*([.\d][.\w]*)\s+([.\d][.\w]*)\s+([.\d][.\w]*)\s+([.\d][.\w]*)/g;
	goog.i18n.bidi.leftRe_ = /left/gi;
	goog.i18n.bidi.rightRe_ = /right/gi;
	goog.i18n.bidi.tempRe_ = /%%%%/g;
	goog.i18n.bidi.mirrorCSS = function (a) {
	    return a.replace(goog.i18n.bidi.dimensionsRe_, ":$1 $4 $3 $2").replace(goog.i18n.bidi.leftRe_, "%%%%").replace(goog.i18n.bidi.rightRe_, goog.i18n.bidi.LEFT).replace(goog.i18n.bidi.tempRe_, goog.i18n.bidi.RIGHT)
	};
	goog.i18n.bidi.doubleQuoteSubstituteRe_ = /([\u0591-\u05f2])"/g;
	goog.i18n.bidi.singleQuoteSubstituteRe_ = /([\u0591-\u05f2])'/g;
	goog.i18n.bidi.normalizeHebrewQuote = function (a) {
	    return a.replace(goog.i18n.bidi.doubleQuoteSubstituteRe_, "$1\u05f4").replace(goog.i18n.bidi.singleQuoteSubstituteRe_, "$1\u05f3")
	};
	goog.i18n.bidi.wordSeparatorRe_ = /\s+/;
	goog.i18n.bidi.hasNumeralsRe_ = /[\d\u06f0-\u06f9]/;
	goog.i18n.bidi.rtlDetectionThreshold_ = .4;
	goog.i18n.bidi.estimateDirection = function (a, b) {
	    var c = 0,
	        d = 0,
	        e = !1;
	    a = goog.i18n.bidi.stripHtmlIfNeeded_(a, b).split(goog.i18n.bidi.wordSeparatorRe_);
	    for (b = 0; b < a.length; b++) {
	        var f = a[b];
	        goog.i18n.bidi.startsWithRtl(f) ? (c++, d++) : goog.i18n.bidi.isRequiredLtrRe_.test(f) ? e = !0 : goog.i18n.bidi.hasAnyLtr(f) ? d++ : goog.i18n.bidi.hasNumeralsRe_.test(f) && (e = !0);
	    }
	    return 0 == d ? e ? goog.i18n.bidi.Dir.LTR : goog.i18n.bidi.Dir.NEUTRAL : c / d > goog.i18n.bidi.rtlDetectionThreshold_ ? goog.i18n.bidi.Dir.RTL : goog.i18n.bidi.Dir.LTR
	};
	goog.i18n.bidi.detectRtlDirectionality = function (a, b) {
	    return goog.i18n.bidi.estimateDirection(a, b) == goog.i18n.bidi.Dir.RTL
	};
	goog.i18n.bidi.setElementDirAndAlign = function (a, b) {
	    a && (b = goog.i18n.bidi.toDir(b)) && (a.style.textAlign = b == goog.i18n.bidi.Dir.RTL ? goog.i18n.bidi.RIGHT : goog.i18n.bidi.LEFT, a.dir = b == goog.i18n.bidi.Dir.RTL ? "rtl" : "ltr");
	};
	goog.i18n.bidi.setElementDirByTextDirectionality = function (a, b) {
	    switch (goog.i18n.bidi.estimateDirection(b)) {
	        case goog.i18n.bidi.Dir.LTR:
	            "ltr" !== a.dir && (a.dir = "ltr");
	            break;
	        case goog.i18n.bidi.Dir.RTL:
	            "rtl" !== a.dir && (a.dir = "rtl");
	            break;
	        default:
	            a.removeAttribute("dir");
	    }
	};
	goog.i18n.bidi.DirectionalString = function () {};
	goog.html.TrustedResourceUrl = function (a, b) {
	    this.privateDoNotAccessOrElseTrustedResourceUrlWrappedValue_ = b === goog.html.TrustedResourceUrl.CONSTRUCTOR_TOKEN_PRIVATE_ ? a : "";
	};
	goog.html.TrustedResourceUrl.prototype.implementsGoogStringTypedString = !0;
	goog.html.TrustedResourceUrl.prototype.getTypedStringValue = function () {
	    return this.privateDoNotAccessOrElseTrustedResourceUrlWrappedValue_.toString()
	};
	goog.html.TrustedResourceUrl.prototype.implementsGoogI18nBidiDirectionalString = !0;
	goog.html.TrustedResourceUrl.prototype.getDirection = function () {
	    return goog.i18n.bidi.Dir.LTR
	};
	goog.html.TrustedResourceUrl.prototype.cloneWithParams = function (a, b) {
	    var c = goog.html.TrustedResourceUrl.unwrap(this);
	    c = goog.html.TrustedResourceUrl.URL_PARAM_PARSER_.exec(c);
	    var d = c[3] || "";
	    return goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse(c[1] + goog.html.TrustedResourceUrl.stringifyParams_("?", c[2] || "", a) + goog.html.TrustedResourceUrl.stringifyParams_("#", d, b))
	};
	goog.html.TrustedResourceUrl.prototype.toString = function () {
	    return this.privateDoNotAccessOrElseTrustedResourceUrlWrappedValue_ + ""
	};
	goog.html.TrustedResourceUrl.unwrap = function (a) {
	    return goog.html.TrustedResourceUrl.unwrapTrustedScriptURL(a).toString()
	};
	goog.html.TrustedResourceUrl.unwrapTrustedScriptURL = function (a) {
	    if (a instanceof goog.html.TrustedResourceUrl && a.constructor === goog.html.TrustedResourceUrl) return a.privateDoNotAccessOrElseTrustedResourceUrlWrappedValue_;
	    goog.asserts.fail("expected object of type TrustedResourceUrl, got '" + a + "' of type " + goog.typeOf(a));
	    return "type_error:TrustedResourceUrl"
	};
	goog.html.TrustedResourceUrl.format = function (a, b) {
	    var c = goog.string.Const.unwrap(a);
	    if (!goog.html.TrustedResourceUrl.BASE_URL_.test(c)) throw Error("Invalid TrustedResourceUrl format: " + c);
	    a = c.replace(goog.html.TrustedResourceUrl.FORMAT_MARKER_, function (d, e) {
	        if (!Object.prototype.hasOwnProperty.call(b, e)) throw Error('Found marker, "' + e + '", in format string, "' + c + '", but no valid label mapping found in args: ' + JSON.stringify(b));
	        d = b[e];
	        return d instanceof goog.string.Const ? goog.string.Const.unwrap(d) :
	            encodeURIComponent(String(d))
	    });
	    return goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.TrustedResourceUrl.FORMAT_MARKER_ = /%{(\w+)}/g;
	goog.html.TrustedResourceUrl.BASE_URL_ = /^((https:)?\/\/[0-9a-z.:[\]-]+\/|\/[^/\\]|[^:/\\%]+\/|[^:/\\%]*[?#]|about:blank#)/i;
	goog.html.TrustedResourceUrl.URL_PARAM_PARSER_ = /^([^?#]*)(\?[^#]*)?(#[\s\S]*)?/;
	goog.html.TrustedResourceUrl.formatWithParams = function (a, b, c, d) {
	    return goog.html.TrustedResourceUrl.format(a, b).cloneWithParams(c, d)
	};
	goog.html.TrustedResourceUrl.fromConstant = function (a) {
	    return goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse(goog.string.Const.unwrap(a))
	};
	goog.html.TrustedResourceUrl.fromConstants = function (a) {
	    for (var b = "", c = 0; c < a.length; c++) b += goog.string.Const.unwrap(a[c]);
	    return goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.html.TrustedResourceUrl.fromSafeScript = function (a) {
	    a = goog.fs.blob.getBlobWithProperties([module$contents$goog$html$SafeScript_SafeScript.unwrap(a)], "text/javascript");
	    a = goog.fs.url.createObjectUrl(a);
	    return goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.TrustedResourceUrl.CONSTRUCTOR_TOKEN_PRIVATE_ = {};
	goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse = function (a) {
	    var b = goog.html.trustedtypes.getPolicyPrivateDoNotAccessOrElse();
	    a = b ? b.createScriptURL(a) : a;
	    return new goog.html.TrustedResourceUrl(a, goog.html.TrustedResourceUrl.CONSTRUCTOR_TOKEN_PRIVATE_)
	};
	goog.html.TrustedResourceUrl.stringifyParams_ = function (a, b, c) {
	    if (null == c) return b;
	    if ("string" === typeof c) return c ? a + encodeURIComponent(c) : "";
	    for (var d in c)
	        if (Object.prototype.hasOwnProperty.call(c, d)) {
	            var e = c[d];
	            e = Array.isArray(e) ? e : [e];
	            for (var f = 0; f < e.length; f++) {
	                var g = e[f];
	                null != g && (b || (b = a), b += (b.length > a.length ? "&" : "") + encodeURIComponent(d) + "=" + encodeURIComponent(String(g)));
	            }
	        } return b
	};
	goog.string.internal = {};
	goog.string.internal.startsWith = function (a, b) {
	    return 0 == a.lastIndexOf(b, 0)
	};
	goog.string.internal.endsWith = function (a, b) {
	    var c = a.length - b.length;
	    return 0 <= c && a.indexOf(b, c) == c
	};
	goog.string.internal.caseInsensitiveStartsWith = function (a, b) {
	    return 0 == goog.string.internal.caseInsensitiveCompare(b, a.substr(0, b.length))
	};
	goog.string.internal.caseInsensitiveEndsWith = function (a, b) {
	    return 0 == goog.string.internal.caseInsensitiveCompare(b, a.substr(a.length - b.length, b.length))
	};
	goog.string.internal.caseInsensitiveEquals = function (a, b) {
	    return a.toLowerCase() == b.toLowerCase()
	};
	goog.string.internal.isEmptyOrWhitespace = function (a) {
	    return /^[\s\xa0]*$/.test(a)
	};
	goog.string.internal.trim = goog.TRUSTED_SITE && String.prototype.trim ? function (a) {
	    return a.trim()
	} : function (a) {
	    return /^[\s\xa0]*([\s\S]*?)[\s\xa0]*$/.exec(a)[1]
	};
	goog.string.internal.caseInsensitiveCompare = function (a, b) {
	    a = String(a).toLowerCase();
	    b = String(b).toLowerCase();
	    return a < b ? -1 : a == b ? 0 : 1
	};
	goog.string.internal.newLineToBr = function (a, b) {
	    return a.replace(/(\r\n|\r|\n)/g, b ? "<br />" : "<br>")
	};
	goog.string.internal.htmlEscape = function (a, b) {
	    if (b) a = a.replace(goog.string.internal.AMP_RE_, "&amp;").replace(goog.string.internal.LT_RE_, "&lt;").replace(goog.string.internal.GT_RE_, "&gt;").replace(goog.string.internal.QUOT_RE_, "&quot;").replace(goog.string.internal.SINGLE_QUOTE_RE_, "&#39;").replace(goog.string.internal.NULL_RE_, "&#0;");
	    else {
	        if (!goog.string.internal.ALL_RE_.test(a)) return a; - 1 != a.indexOf("&") && (a = a.replace(goog.string.internal.AMP_RE_, "&amp;")); - 1 != a.indexOf("<") && (a = a.replace(goog.string.internal.LT_RE_,
	            "&lt;")); - 1 != a.indexOf(">") && (a = a.replace(goog.string.internal.GT_RE_, "&gt;")); - 1 != a.indexOf('"') && (a = a.replace(goog.string.internal.QUOT_RE_, "&quot;")); - 1 != a.indexOf("'") && (a = a.replace(goog.string.internal.SINGLE_QUOTE_RE_, "&#39;")); - 1 != a.indexOf("\x00") && (a = a.replace(goog.string.internal.NULL_RE_, "&#0;"));
	    }
	    return a
	};
	goog.string.internal.AMP_RE_ = /&/g;
	goog.string.internal.LT_RE_ = /</g;
	goog.string.internal.GT_RE_ = />/g;
	goog.string.internal.QUOT_RE_ = /"/g;
	goog.string.internal.SINGLE_QUOTE_RE_ = /'/g;
	goog.string.internal.NULL_RE_ = /\x00/g;
	goog.string.internal.ALL_RE_ = /[\x00&<>"']/;
	goog.string.internal.whitespaceEscape = function (a, b) {
	    return goog.string.internal.newLineToBr(a.replace(/  /g, " &#160;"), b)
	};
	goog.string.internal.contains = function (a, b) {
	    return -1 != a.indexOf(b)
	};
	goog.string.internal.caseInsensitiveContains = function (a, b) {
	    return goog.string.internal.contains(a.toLowerCase(), b.toLowerCase())
	};
	goog.string.internal.compareVersions = function (a, b) {
	    var c = 0;
	    a = goog.string.internal.trim(String(a)).split(".");
	    b = goog.string.internal.trim(String(b)).split(".");
	    for (var d = Math.max(a.length, b.length), e = 0; 0 == c && e < d; e++) {
	        var f = a[e] || "",
	            g = b[e] || "";
	        do {
	            f = /(\d*)(\D*)(.*)/.exec(f) || ["", "", "", ""];
	            g = /(\d*)(\D*)(.*)/.exec(g) || ["", "", "", ""];
	            if (0 == f[0].length && 0 == g[0].length) break;
	            c = goog.string.internal.compareElements_(0 == f[1].length ? 0 : parseInt(f[1], 10), 0 == g[1].length ? 0 : parseInt(g[1], 10)) || goog.string.internal.compareElements_(0 ==
	                f[2].length, 0 == g[2].length) || goog.string.internal.compareElements_(f[2], g[2]);
	            f = f[3];
	            g = g[3];
	        } while (0 == c)
	    }
	    return c
	};
	goog.string.internal.compareElements_ = function (a, b) {
	    return a < b ? -1 : a > b ? 1 : 0
	};
	goog.html.SafeUrl = function (a, b) {
	    this.privateDoNotAccessOrElseSafeUrlWrappedValue_ = b === goog.html.SafeUrl.CONSTRUCTOR_TOKEN_PRIVATE_ ? a : "";
	};
	goog.html.SafeUrl.INNOCUOUS_STRING = "about:invalid#zClosurez";
	goog.html.SafeUrl.prototype.implementsGoogStringTypedString = !0;
	goog.html.SafeUrl.prototype.getTypedStringValue = function () {
	    return this.privateDoNotAccessOrElseSafeUrlWrappedValue_.toString()
	};
	goog.html.SafeUrl.prototype.implementsGoogI18nBidiDirectionalString = !0;
	goog.html.SafeUrl.prototype.getDirection = function () {
	    return goog.i18n.bidi.Dir.LTR
	};
	goog.html.SafeUrl.prototype.toString = function () {
	    return this.privateDoNotAccessOrElseSafeUrlWrappedValue_.toString()
	};
	goog.html.SafeUrl.unwrap = function (a) {
	    if (a instanceof goog.html.SafeUrl && a.constructor === goog.html.SafeUrl) return a.privateDoNotAccessOrElseSafeUrlWrappedValue_;
	    goog.asserts.fail("expected object of type SafeUrl, got '" + a + "' of type " + goog.typeOf(a));
	    return "type_error:SafeUrl"
	};
	goog.html.SafeUrl.fromConstant = function (a) {
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(goog.string.Const.unwrap(a))
	};
	goog.html.SAFE_MIME_TYPE_PATTERN_ = /^(?:audio\/(?:3gpp2|3gpp|aac|L16|midi|mp3|mp4|mpeg|oga|ogg|opus|x-m4a|x-matroska|x-wav|wav|webm)|font\/\w+|image\/(?:bmp|gif|jpeg|jpg|png|tiff|webp|x-icon)|video\/(?:mpeg|mp4|ogg|webm|quicktime|x-matroska))(?:;\w+=(?:\w+|"[\w;,= ]+"))*$/i;
	goog.html.SafeUrl.isSafeMimeType = function (a) {
	    return goog.html.SAFE_MIME_TYPE_PATTERN_.test(a)
	};
	goog.html.SafeUrl.fromBlob = function (a) {
	    a = goog.html.SafeUrl.isSafeMimeType(a.type) ? goog.fs.url.createObjectUrl(a) : goog.html.SafeUrl.INNOCUOUS_STRING;
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.revokeObjectUrl = function (a) {
	    a = a.getTypedStringValue();
	    a !== goog.html.SafeUrl.INNOCUOUS_STRING && goog.fs.url.revokeObjectUrl(a);
	};
	goog.html.SafeUrl.fromMediaSource = function (a) {
	    goog.asserts.assert("MediaSource" in goog.global, "No support for MediaSource");
	    a = a instanceof MediaSource ? goog.fs.url.createObjectUrl(a) : goog.html.SafeUrl.INNOCUOUS_STRING;
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.DATA_URL_PATTERN_ = /^data:(.*);base64,[a-z0-9+\/]+=*$/i;
	goog.html.SafeUrl.tryFromDataUrl = function (a) {
	    a = String(a);
	    a = a.replace(/(%0A|%0D)/g, "");
	    var b = a.match(goog.html.DATA_URL_PATTERN_);
	    return b && goog.html.SafeUrl.isSafeMimeType(b[1]) ? goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a) : null
	};
	goog.html.SafeUrl.fromDataUrl = function (a) {
	    return goog.html.SafeUrl.tryFromDataUrl(a) || goog.html.SafeUrl.INNOCUOUS_URL
	};
	goog.html.SafeUrl.fromTelUrl = function (a) {
	    goog.string.internal.caseInsensitiveStartsWith(a, "tel:") || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SIP_URL_PATTERN_ = /^sip[s]?:[+a-z0-9_.!$%&'*\/=^`{|}~-]+@([a-z0-9-]+\.)+[a-z0-9]{2,63}$/i;
	goog.html.SafeUrl.fromSipUrl = function (a) {
	    goog.html.SIP_URL_PATTERN_.test(decodeURIComponent(a)) || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.fromFacebookMessengerUrl = function (a) {
	    goog.string.internal.caseInsensitiveStartsWith(a, "fb-messenger://share") || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.fromWhatsAppUrl = function (a) {
	    goog.string.internal.caseInsensitiveStartsWith(a, "whatsapp://send") || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.fromSmsUrl = function (a) {
	    goog.string.internal.caseInsensitiveStartsWith(a, "sms:") && goog.html.SafeUrl.isSmsUrlBodyValid_(a) || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.isSmsUrlBodyValid_ = function (a) {
	    var b = a.indexOf("#");
	    0 < b && (a = a.substring(0, b));
	    b = a.match(/[?&]body=/gi);
	    if (!b) return !0;
	    if (1 < b.length) return !1;
	    a = a.match(/[?&]body=([^&]*)/)[1];
	    if (!a) return !0;
	    try {
	        decodeURIComponent(a);
	    } catch (c) {
	        return !1
	    }
	    return /^(?:[a-z0-9\-_.~]|%[0-9a-f]{2})+$/i.test(a)
	};
	goog.html.SafeUrl.fromSshUrl = function (a) {
	    goog.string.internal.caseInsensitiveStartsWith(a, "ssh://") || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.sanitizeChromeExtensionUrl = function (a, b) {
	    return goog.html.SafeUrl.sanitizeExtensionUrl_(/^chrome-extension:\/\/([^\/]+)\//, a, b)
	};
	goog.html.SafeUrl.sanitizeFirefoxExtensionUrl = function (a, b) {
	    return goog.html.SafeUrl.sanitizeExtensionUrl_(/^moz-extension:\/\/([^\/]+)\//, a, b)
	};
	goog.html.SafeUrl.sanitizeEdgeExtensionUrl = function (a, b) {
	    return goog.html.SafeUrl.sanitizeExtensionUrl_(/^ms-browser-extension:\/\/([^\/]+)\//, a, b)
	};
	goog.html.SafeUrl.sanitizeExtensionUrl_ = function (a, b, c) {
	    (a = a.exec(b)) ? (a = a[1], -1 == (c instanceof goog.string.Const ? [goog.string.Const.unwrap(c)] : c.map(function (d) {
	        return goog.string.Const.unwrap(d)
	    })).indexOf(a) && (b = goog.html.SafeUrl.INNOCUOUS_STRING)) : b = goog.html.SafeUrl.INNOCUOUS_STRING;
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.html.SafeUrl.fromTrustedResourceUrl = function (a) {
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(goog.html.TrustedResourceUrl.unwrap(a))
	};
	goog.html.SAFE_URL_PATTERN_ = /^(?:(?:https?|mailto|ftp):|[^:/?#]*(?:[/?#]|$))/i;
	goog.html.SafeUrl.SAFE_URL_PATTERN = goog.html.SAFE_URL_PATTERN_;
	goog.html.SafeUrl.trySanitize = function (a) {
	    if (a instanceof goog.html.SafeUrl) return a;
	    a = "object" == typeof a && a.implementsGoogStringTypedString ? a.getTypedStringValue() : String(a);
	    return goog.html.SAFE_URL_PATTERN_.test(a) ? goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a) : goog.html.SafeUrl.tryFromDataUrl(a)
	};
	goog.html.SafeUrl.sanitize = function (a) {
	    return goog.html.SafeUrl.trySanitize(a) || goog.html.SafeUrl.INNOCUOUS_URL
	};
	goog.html.SafeUrl.sanitizeAssertUnchanged = function (a, b) {
	    if (a instanceof goog.html.SafeUrl) return a;
	    a = "object" == typeof a && a.implementsGoogStringTypedString ? a.getTypedStringValue() : String(a);
	    if (b && /^data:/i.test(a) && (b = goog.html.SafeUrl.fromDataUrl(a), b.getTypedStringValue() == a)) return b;
	    goog.asserts.assert(goog.html.SAFE_URL_PATTERN_.test(a), "%s does not match the safe URL pattern", a) || (a = goog.html.SafeUrl.INNOCUOUS_STRING);
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeUrl.CONSTRUCTOR_TOKEN_PRIVATE_ = {};
	goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse = function (a) {
	    return new goog.html.SafeUrl(a, goog.html.SafeUrl.CONSTRUCTOR_TOKEN_PRIVATE_)
	};
	goog.html.SafeUrl.INNOCUOUS_URL = goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(goog.html.SafeUrl.INNOCUOUS_STRING);
	goog.html.SafeUrl.ABOUT_BLANK = goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse("about:blank");
	goog.html.SafeStyle = function (a, b) {
	    this.privateDoNotAccessOrElseSafeStyleWrappedValue_ = b === goog.html.SafeStyle.CONSTRUCTOR_TOKEN_PRIVATE_ ? a : "";
	};
	goog.html.SafeStyle.prototype.implementsGoogStringTypedString = !0;
	goog.html.SafeStyle.fromConstant = function (a) {
	    a = goog.string.Const.unwrap(a);
	    if (0 === a.length) return goog.html.SafeStyle.EMPTY;
	    goog.asserts.assert(goog.string.internal.endsWith(a, ";"), "Last character of style string is not ';': " + a);
	    goog.asserts.assert(goog.string.internal.contains(a, ":"), "Style string must contain at least one ':', to specify a \"name: value\" pair: " + a);
	    return goog.html.SafeStyle.createSafeStyleSecurityPrivateDoNotAccessOrElse(a)
	};
	goog.html.SafeStyle.prototype.getTypedStringValue = function () {
	    return this.privateDoNotAccessOrElseSafeStyleWrappedValue_
	};
	goog.html.SafeStyle.prototype.toString = function () {
	    return this.privateDoNotAccessOrElseSafeStyleWrappedValue_.toString()
	};
	goog.html.SafeStyle.unwrap = function (a) {
	    if (a instanceof goog.html.SafeStyle && a.constructor === goog.html.SafeStyle) return a.privateDoNotAccessOrElseSafeStyleWrappedValue_;
	    goog.asserts.fail("expected object of type SafeStyle, got '" + a + "' of type " + goog.typeOf(a));
	    return "type_error:SafeStyle"
	};
	goog.html.SafeStyle.CONSTRUCTOR_TOKEN_PRIVATE_ = {};
	goog.html.SafeStyle.createSafeStyleSecurityPrivateDoNotAccessOrElse = function (a) {
	    return new goog.html.SafeStyle(a, goog.html.SafeStyle.CONSTRUCTOR_TOKEN_PRIVATE_)
	};
	goog.html.SafeStyle.EMPTY = goog.html.SafeStyle.createSafeStyleSecurityPrivateDoNotAccessOrElse("");
	goog.html.SafeStyle.INNOCUOUS_STRING = "zClosurez";
	goog.html.SafeStyle.create = function (a) {
	    var b = "",
	        c;
	    for (c in a)
	        if (Object.prototype.hasOwnProperty.call(a, c)) {
	            if (!/^[-_a-zA-Z0-9]+$/.test(c)) throw Error("Name allows only [-_a-zA-Z0-9], got: " + c);
	            var d = a[c];
	            null != d && (d = Array.isArray(d) ? module$contents$goog$array_map(d, goog.html.SafeStyle.sanitizePropertyValue_).join(" ") : goog.html.SafeStyle.sanitizePropertyValue_(d), b += c + ":" + d + ";");
	        } return b ? goog.html.SafeStyle.createSafeStyleSecurityPrivateDoNotAccessOrElse(b) : goog.html.SafeStyle.EMPTY
	};
	goog.html.SafeStyle.sanitizePropertyValue_ = function (a) {
	    if (a instanceof goog.html.SafeUrl) return 'url("' + goog.html.SafeUrl.unwrap(a).replace(/</g, "%3c").replace(/[\\"]/g, "\\$&") + '")';
	    a = a instanceof goog.string.Const ? goog.string.Const.unwrap(a) : goog.html.SafeStyle.sanitizePropertyValueString_(String(a));
	    if (/[{;}]/.test(a)) throw new goog.asserts.AssertionError("Value does not allow [{;}], got: %s.", [a]);
	    return a
	};
	goog.html.SafeStyle.sanitizePropertyValueString_ = function (a) {
	    var b = a.replace(goog.html.SafeStyle.FUNCTIONS_RE_, "$1").replace(goog.html.SafeStyle.FUNCTIONS_RE_, "$1").replace(goog.html.SafeStyle.URL_RE_, "url");
	    if (goog.html.SafeStyle.VALUE_RE_.test(b)) {
	        if (goog.html.SafeStyle.COMMENT_RE_.test(a)) return goog.asserts.fail("String value disallows comments, got: " + a), goog.html.SafeStyle.INNOCUOUS_STRING;
	        if (!goog.html.SafeStyle.hasBalancedQuotes_(a)) return goog.asserts.fail("String value requires balanced quotes, got: " +
	            a), goog.html.SafeStyle.INNOCUOUS_STRING;
	        if (!goog.html.SafeStyle.hasBalancedSquareBrackets_(a)) return goog.asserts.fail("String value requires balanced square brackets and one identifier per pair of brackets, got: " + a), goog.html.SafeStyle.INNOCUOUS_STRING
	    } else return goog.asserts.fail("String value allows only " + goog.html.SafeStyle.VALUE_ALLOWED_CHARS_ + " and simple functions, got: " + a), goog.html.SafeStyle.INNOCUOUS_STRING;
	    return goog.html.SafeStyle.sanitizeUrl_(a)
	};
	goog.html.SafeStyle.hasBalancedQuotes_ = function (a) {
	    for (var b = !0, c = !0, d = 0; d < a.length; d++) {
	        var e = a.charAt(d);
	        "'" == e && c ? b = !b : '"' == e && b && (c = !c);
	    }
	    return b && c
	};
	goog.html.SafeStyle.hasBalancedSquareBrackets_ = function (a) {
	    for (var b = !0, c = /^[-_a-zA-Z0-9]$/, d = 0; d < a.length; d++) {
	        var e = a.charAt(d);
	        if ("]" == e) {
	            if (b) return !1;
	            b = !0;
	        } else if ("[" == e) {
	            if (!b) return !1;
	            b = !1;
	        } else if (!b && !c.test(e)) return !1
	    }
	    return b
	};
	goog.html.SafeStyle.VALUE_ALLOWED_CHARS_ = "[-,.\"'%_!# a-zA-Z0-9\\[\\]]";
	goog.html.SafeStyle.VALUE_RE_ = new RegExp("^" + goog.html.SafeStyle.VALUE_ALLOWED_CHARS_ + "+$");
	goog.html.SafeStyle.URL_RE_ = /\b(url\([ \t\n]*)('[ -&(-\[\]-~]*'|"[ !#-\[\]-~]*"|[!#-&*-\[\]-~]*)([ \t\n]*\))/g;
	goog.html.SafeStyle.ALLOWED_FUNCTIONS_ = "calc cubic-bezier fit-content hsl hsla linear-gradient matrix minmax repeat rgb rgba (rotate|scale|translate)(X|Y|Z|3d)?".split(" ");
	goog.html.SafeStyle.FUNCTIONS_RE_ = new RegExp("\\b(" + goog.html.SafeStyle.ALLOWED_FUNCTIONS_.join("|") + ")\\([-+*/0-9a-z.%\\[\\], ]+\\)", "g");
	goog.html.SafeStyle.COMMENT_RE_ = /\/\*/;
	goog.html.SafeStyle.sanitizeUrl_ = function (a) {
	    return a.replace(goog.html.SafeStyle.URL_RE_, function (b, c, d, e) {
	        var f = "";
	        d = d.replace(/^(['"])(.*)\1$/, function (g, h, k) {
	            f = h;
	            return k
	        });
	        b = goog.html.SafeUrl.sanitize(d).getTypedStringValue();
	        return c + f + b + f + e
	    })
	};
	goog.html.SafeStyle.concat = function (a) {
	    var b = "",
	        c = function (d) {
	            Array.isArray(d) ? module$contents$goog$array_forEach(d, c) : b += goog.html.SafeStyle.unwrap(d);
	        };
	    module$contents$goog$array_forEach(arguments, c);
	    return b ? goog.html.SafeStyle.createSafeStyleSecurityPrivateDoNotAccessOrElse(b) : goog.html.SafeStyle.EMPTY
	};
	var module$contents$goog$html$SafeStyleSheet_CONSTRUCTOR_TOKEN_PRIVATE = {},
	    module$contents$goog$html$SafeStyleSheet_SafeStyleSheet = function (a, b) {
	        this.privateDoNotAccessOrElseSafeStyleSheetWrappedValue_ = b === module$contents$goog$html$SafeStyleSheet_CONSTRUCTOR_TOKEN_PRIVATE ? a : "";
	        this.implementsGoogStringTypedString = !0;
	    };
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createRule = function (a, b) {
	    if ((0, goog.string.internal.contains)(a, "<")) throw Error("Selector does not allow '<', got: " + a);
	    var c = a.replace(/('|")((?!\1)[^\r\n\f\\]|\\[\s\S])*\1/g, "");
	    if (!/^[-_a-zA-Z0-9#.:* ,>+~[\]()=^$|]+$/.test(c)) throw Error("Selector allows only [-_a-zA-Z0-9#.:* ,>+~[\\]()=^$|] and strings, got: " + a);
	    if (!module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.hasBalancedBrackets_(c)) throw Error("() and [] in selector must be balanced, got: " +
	        a);
	    b instanceof goog.html.SafeStyle || (b = goog.html.SafeStyle.create(b));
	    a = a + "{" + goog.html.SafeStyle.unwrap(b).replace(/</g, "\\3C ") + "}";
	    return module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createSafeStyleSheetSecurityPrivateDoNotAccessOrElse(a)
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.hasBalancedBrackets_ = function (a) {
	    for (var b = {
	            "(": ")",
	            "[": "]"
	        }, c = [], d = 0; d < a.length; d++) {
	        var e = a[d];
	        if (b[e]) c.push(b[e]);
	        else if (goog.object.contains(b, e) && c.pop() != e) return !1
	    }
	    return 0 == c.length
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.concat = function (a) {
	    var b = "",
	        c = function (d) {
	            Array.isArray(d) ? module$contents$goog$array_forEach(d, c) : b += module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.unwrap(d);
	        };
	    module$contents$goog$array_forEach(arguments, c);
	    return module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createSafeStyleSheetSecurityPrivateDoNotAccessOrElse(b)
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.fromConstant = function (a) {
	    a = goog.string.Const.unwrap(a);
	    if (0 === a.length) return module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.EMPTY;
	    (0, goog.asserts.assert)(!(0, goog.string.internal.contains)(a, "<"), "Forbidden '<' character in style sheet string: " + a);
	    return module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createSafeStyleSheetSecurityPrivateDoNotAccessOrElse(a)
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.prototype.getTypedStringValue = function () {
	    return this.privateDoNotAccessOrElseSafeStyleSheetWrappedValue_
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.unwrap = function (a) {
	    if (a instanceof module$contents$goog$html$SafeStyleSheet_SafeStyleSheet && a.constructor === module$contents$goog$html$SafeStyleSheet_SafeStyleSheet) return a.privateDoNotAccessOrElseSafeStyleSheetWrappedValue_;
	    (0, goog.asserts.fail)("expected object of type SafeStyleSheet, got '" + a + "' of type " + goog.typeOf(a));
	    return "type_error:SafeStyleSheet"
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createSafeStyleSheetSecurityPrivateDoNotAccessOrElse = function (a) {
	    return new module$contents$goog$html$SafeStyleSheet_SafeStyleSheet(a, module$contents$goog$html$SafeStyleSheet_CONSTRUCTOR_TOKEN_PRIVATE)
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.prototype.toString = function () {
	    return this.privateDoNotAccessOrElseSafeStyleSheetWrappedValue_.toString()
	};
	module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.EMPTY = module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createSafeStyleSheetSecurityPrivateDoNotAccessOrElse("");
	goog.html.SafeStyleSheet = module$contents$goog$html$SafeStyleSheet_SafeStyleSheet;
	goog.labs = {};
	goog.labs.userAgent = {};
	goog.labs.userAgent.util = {};
	goog.labs.userAgent.util.getNativeUserAgentString_ = function () {
	    var a = goog.labs.userAgent.util.getNavigator_();
	    return a && (a = a.userAgent) ? a : ""
	};
	goog.labs.userAgent.util.getNavigator_ = function () {
	    return goog.global.navigator
	};
	goog.labs.userAgent.util.userAgent_ = goog.labs.userAgent.util.getNativeUserAgentString_();
	goog.labs.userAgent.util.setUserAgent = function (a) {
	    goog.labs.userAgent.util.userAgent_ = a || goog.labs.userAgent.util.getNativeUserAgentString_();
	};
	goog.labs.userAgent.util.getUserAgent = function () {
	    return goog.labs.userAgent.util.userAgent_
	};
	goog.labs.userAgent.util.matchUserAgent = function (a) {
	    return goog.string.internal.contains(goog.labs.userAgent.util.getUserAgent(), a)
	};
	goog.labs.userAgent.util.matchUserAgentIgnoreCase = function (a) {
	    return goog.string.internal.caseInsensitiveContains(goog.labs.userAgent.util.getUserAgent(), a)
	};
	goog.labs.userAgent.util.extractVersionTuples = function (a) {
	    for (var b = /(\w[\w ]+)\/([^\s]+)\s*(?:\((.*?)\))?/g, c = [], d; d = b.exec(a);) c.push([d[1], d[2], d[3] || void 0]);
	    return c
	};
	goog.labs.userAgent.browser = {};
	goog.labs.userAgent.browser.matchOpera_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Opera")
	};
	goog.labs.userAgent.browser.matchIE_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Trident") || goog.labs.userAgent.util.matchUserAgent("MSIE")
	};
	goog.labs.userAgent.browser.matchEdgeHtml_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Edge")
	};
	goog.labs.userAgent.browser.matchEdgeChromium_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Edg/")
	};
	goog.labs.userAgent.browser.matchOperaChromium_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("OPR")
	};
	goog.labs.userAgent.browser.matchFirefox_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Firefox") || goog.labs.userAgent.util.matchUserAgent("FxiOS")
	};
	goog.labs.userAgent.browser.matchSafari_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Safari") && !(goog.labs.userAgent.browser.matchChrome_() || goog.labs.userAgent.browser.matchCoast_() || goog.labs.userAgent.browser.matchOpera_() || goog.labs.userAgent.browser.matchEdgeHtml_() || goog.labs.userAgent.browser.matchEdgeChromium_() || goog.labs.userAgent.browser.matchOperaChromium_() || goog.labs.userAgent.browser.matchFirefox_() || goog.labs.userAgent.browser.isSilk() || goog.labs.userAgent.util.matchUserAgent("Android"))
	};
	goog.labs.userAgent.browser.matchCoast_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Coast")
	};
	goog.labs.userAgent.browser.matchIosWebview_ = function () {
	    return (goog.labs.userAgent.util.matchUserAgent("iPad") || goog.labs.userAgent.util.matchUserAgent("iPhone")) && !goog.labs.userAgent.browser.matchSafari_() && !goog.labs.userAgent.browser.matchChrome_() && !goog.labs.userAgent.browser.matchCoast_() && !goog.labs.userAgent.browser.matchFirefox_() && goog.labs.userAgent.util.matchUserAgent("AppleWebKit")
	};
	goog.labs.userAgent.browser.matchChrome_ = function () {
	    return (goog.labs.userAgent.util.matchUserAgent("Chrome") || goog.labs.userAgent.util.matchUserAgent("CriOS")) && !goog.labs.userAgent.browser.matchEdgeHtml_()
	};
	goog.labs.userAgent.browser.matchAndroidBrowser_ = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Android") && !(goog.labs.userAgent.browser.isChrome() || goog.labs.userAgent.browser.isFirefox() || goog.labs.userAgent.browser.isOpera() || goog.labs.userAgent.browser.isSilk())
	};
	goog.labs.userAgent.browser.isOpera = goog.labs.userAgent.browser.matchOpera_;
	goog.labs.userAgent.browser.isIE = goog.labs.userAgent.browser.matchIE_;
	goog.labs.userAgent.browser.isEdge = goog.labs.userAgent.browser.matchEdgeHtml_;
	goog.labs.userAgent.browser.isEdgeChromium = goog.labs.userAgent.browser.matchEdgeChromium_;
	goog.labs.userAgent.browser.isOperaChromium = goog.labs.userAgent.browser.matchOperaChromium_;
	goog.labs.userAgent.browser.isFirefox = goog.labs.userAgent.browser.matchFirefox_;
	goog.labs.userAgent.browser.isSafari = goog.labs.userAgent.browser.matchSafari_;
	goog.labs.userAgent.browser.isCoast = goog.labs.userAgent.browser.matchCoast_;
	goog.labs.userAgent.browser.isIosWebview = goog.labs.userAgent.browser.matchIosWebview_;
	goog.labs.userAgent.browser.isChrome = goog.labs.userAgent.browser.matchChrome_;
	goog.labs.userAgent.browser.isAndroidBrowser = goog.labs.userAgent.browser.matchAndroidBrowser_;
	goog.labs.userAgent.browser.isSilk = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Silk")
	};
	goog.labs.userAgent.browser.getVersion = function () {
	    function a(e) {
	        e = module$contents$goog$array_find(e, d);
	        return c[e] || ""
	    }
	    var b = goog.labs.userAgent.util.getUserAgent();
	    if (goog.labs.userAgent.browser.isIE()) return goog.labs.userAgent.browser.getIEVersion_(b);
	    b = goog.labs.userAgent.util.extractVersionTuples(b);
	    var c = {};
	    module$contents$goog$array_forEach(b, function (e) {
	        c[e[0]] = e[1];
	    });
	    var d = goog.partial(goog.object.containsKey, c);
	    return goog.labs.userAgent.browser.isOpera() ? a(["Version", "Opera"]) : goog.labs.userAgent.browser.isEdge() ?
	        a(["Edge"]) : goog.labs.userAgent.browser.isEdgeChromium() ? a(["Edg"]) : goog.labs.userAgent.browser.isChrome() ? a(["Chrome", "CriOS", "HeadlessChrome"]) : (b = b[2]) && b[1] || ""
	};
	goog.labs.userAgent.browser.isVersionOrHigher = function (a) {
	    return 0 <= goog.string.internal.compareVersions(goog.labs.userAgent.browser.getVersion(), a)
	};
	goog.labs.userAgent.browser.getIEVersion_ = function (a) {
	    var b = /rv: *([\d\.]*)/.exec(a);
	    if (b && b[1]) return b[1];
	    b = "";
	    var c = /MSIE +([\d\.]+)/.exec(a);
	    if (c && c[1])
	        if (a = /Trident\/(\d.\d)/.exec(a), "7.0" == c[1])
	            if (a && a[1]) switch (a[1]) {
	                case "4.0":
	                    b = "8.0";
	                    break;
	                case "5.0":
	                    b = "9.0";
	                    break;
	                case "6.0":
	                    b = "10.0";
	                    break;
	                case "7.0":
	                    b = "11.0";
	            } else b = "7.0";
	            else b = c[1];
	    return b
	};
	goog.html.SafeHtml = function (a, b, c) {
	    this.privateDoNotAccessOrElseSafeHtmlWrappedValue_ = c === goog.html.SafeHtml.CONSTRUCTOR_TOKEN_PRIVATE_ ? a : "";
	    this.dir_ = b;
	};
	goog.html.SafeHtml.ENABLE_ERROR_MESSAGES = goog.DEBUG;
	goog.html.SafeHtml.SUPPORT_STYLE_ATTRIBUTE = !0;
	goog.html.SafeHtml.prototype.implementsGoogI18nBidiDirectionalString = !0;
	goog.html.SafeHtml.prototype.getDirection = function () {
	    return this.dir_
	};
	goog.html.SafeHtml.prototype.implementsGoogStringTypedString = !0;
	goog.html.SafeHtml.prototype.getTypedStringValue = function () {
	    return this.privateDoNotAccessOrElseSafeHtmlWrappedValue_.toString()
	};
	goog.html.SafeHtml.prototype.toString = function () {
	    return this.privateDoNotAccessOrElseSafeHtmlWrappedValue_.toString()
	};
	goog.html.SafeHtml.unwrap = function (a) {
	    return goog.html.SafeHtml.unwrapTrustedHTML(a).toString()
	};
	goog.html.SafeHtml.unwrapTrustedHTML = function (a) {
	    if (a instanceof goog.html.SafeHtml && a.constructor === goog.html.SafeHtml) return a.privateDoNotAccessOrElseSafeHtmlWrappedValue_;
	    goog.asserts.fail("expected object of type SafeHtml, got '" + a + "' of type " + goog.typeOf(a));
	    return "type_error:SafeHtml"
	};
	goog.html.SafeHtml.htmlEscape = function (a) {
	    if (a instanceof goog.html.SafeHtml) return a;
	    var b = "object" == typeof a,
	        c = null;
	    b && a.implementsGoogI18nBidiDirectionalString && (c = a.getDirection());
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(goog.string.internal.htmlEscape(b && a.implementsGoogStringTypedString ? a.getTypedStringValue() : String(a)), c)
	};
	goog.html.SafeHtml.htmlEscapePreservingNewlines = function (a) {
	    if (a instanceof goog.html.SafeHtml) return a;
	    a = goog.html.SafeHtml.htmlEscape(a);
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(goog.string.internal.newLineToBr(goog.html.SafeHtml.unwrap(a)), a.getDirection())
	};
	goog.html.SafeHtml.htmlEscapePreservingNewlinesAndSpaces = function (a) {
	    if (a instanceof goog.html.SafeHtml) return a;
	    a = goog.html.SafeHtml.htmlEscape(a);
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(goog.string.internal.whitespaceEscape(goog.html.SafeHtml.unwrap(a)), a.getDirection())
	};
	goog.html.SafeHtml.from = goog.html.SafeHtml.htmlEscape;
	goog.html.SafeHtml.comment = function (a) {
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse("\x3c!--" + goog.string.internal.htmlEscape(a) + "--\x3e", null)
	};
	goog.html.SafeHtml.VALID_NAMES_IN_TAG_ = /^[a-zA-Z0-9-]+$/;
	goog.html.SafeHtml.URL_ATTRIBUTES_ = {
	    action: !0,
	    cite: !0,
	    data: !0,
	    formaction: !0,
	    href: !0,
	    manifest: !0,
	    poster: !0,
	    src: !0
	};
	goog.html.SafeHtml.NOT_ALLOWED_TAG_NAMES_ = goog.object.createSet(goog.dom.TagName.APPLET, goog.dom.TagName.BASE, goog.dom.TagName.EMBED, goog.dom.TagName.IFRAME, goog.dom.TagName.LINK, goog.dom.TagName.MATH, goog.dom.TagName.META, goog.dom.TagName.OBJECT, goog.dom.TagName.SCRIPT, goog.dom.TagName.STYLE, goog.dom.TagName.SVG, goog.dom.TagName.TEMPLATE);
	goog.html.SafeHtml.create = function (a, b, c) {
	    goog.html.SafeHtml.verifyTagName(String(a));
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse(String(a), b, c)
	};
	goog.html.SafeHtml.verifyTagName = function (a) {
	    if (!goog.html.SafeHtml.VALID_NAMES_IN_TAG_.test(a)) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? "Invalid tag name <" + a + ">." : "");
	    if (a.toUpperCase() in goog.html.SafeHtml.NOT_ALLOWED_TAG_NAMES_) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? "Tag name <" + a + "> is not allowed for SafeHtml." : "");
	};
	goog.html.SafeHtml.createIframe = function (a, b, c, d) {
	    a && goog.html.TrustedResourceUrl.unwrap(a);
	    var e = {};
	    e.src = a || null;
	    e.srcdoc = b && goog.html.SafeHtml.unwrap(b);
	    a = goog.html.SafeHtml.combineAttributes(e, {
	        sandbox: ""
	    }, c);
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse("iframe", a, d)
	};
	goog.html.SafeHtml.createSandboxIframe = function (a, b, c, d) {
	    if (!goog.html.SafeHtml.canUseSandboxIframe()) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? "The browser does not support sandboxed iframes." : "");
	    var e = {};
	    e.src = a ? goog.html.SafeUrl.unwrap(goog.html.SafeUrl.sanitize(a)) : null;
	    e.srcdoc = b || null;
	    e.sandbox = "";
	    a = goog.html.SafeHtml.combineAttributes(e, {}, c);
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse("iframe", a, d)
	};
	goog.html.SafeHtml.canUseSandboxIframe = function () {
	    return goog.global.HTMLIFrameElement && "sandbox" in goog.global.HTMLIFrameElement.prototype
	};
	goog.html.SafeHtml.createScriptSrc = function (a, b) {
	    goog.html.TrustedResourceUrl.unwrap(a);
	    a = goog.html.SafeHtml.combineAttributes({
	        src: a
	    }, {}, b);
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse("script", a)
	};
	goog.html.SafeHtml.createScript = function (a, b) {
	    for (var c in b)
	        if (Object.prototype.hasOwnProperty.call(b, c)) {
	            var d = c.toLowerCase();
	            if ("language" == d || "src" == d || "text" == d || "type" == d) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'Cannot set "' + d + '" attribute' : "");
	        } c = "";
	    a = module$contents$goog$array_concat(a);
	    for (d = 0; d < a.length; d++) c += module$contents$goog$html$SafeScript_SafeScript.unwrap(a[d]);
	    a = goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(c, goog.i18n.bidi.Dir.NEUTRAL);
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse("script",
	        b, a)
	};
	goog.html.SafeHtml.createStyle = function (a, b) {
	    b = goog.html.SafeHtml.combineAttributes({
	        type: "text/css"
	    }, {}, b);
	    var c = "";
	    a = module$contents$goog$array_concat(a);
	    for (var d = 0; d < a.length; d++) c += module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.unwrap(a[d]);
	    a = goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(c, goog.i18n.bidi.Dir.NEUTRAL);
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse("style", b, a)
	};
	goog.html.SafeHtml.createMetaRefresh = function (a, b) {
	    a = goog.html.SafeUrl.unwrap(goog.html.SafeUrl.sanitize(a));
	    (goog.labs.userAgent.browser.isIE() || goog.labs.userAgent.browser.isEdge()) && goog.string.internal.contains(a, ";") && (a = "'" + a.replace(/'/g, "%27") + "'");
	    return goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse("meta", {
	        "http-equiv": "refresh",
	        content: (b || 0) + "; url=" + a
	    })
	};
	goog.html.SafeHtml.getAttrNameAndValue_ = function (a, b, c) {
	    if (c instanceof goog.string.Const) c = goog.string.Const.unwrap(c);
	    else if ("style" == b.toLowerCase())
	        if (goog.html.SafeHtml.SUPPORT_STYLE_ATTRIBUTE) c = goog.html.SafeHtml.getStyleValue_(c);
	        else throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'Attribute "style" not supported.' : "");
	    else {
	        if (/^on/i.test(b)) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'Attribute "' + b + '" requires goog.string.Const value, "' + c + '" given.' : "");
	        if (b.toLowerCase() in
	            goog.html.SafeHtml.URL_ATTRIBUTES_)
	            if (c instanceof goog.html.TrustedResourceUrl) c = goog.html.TrustedResourceUrl.unwrap(c);
	            else if (c instanceof goog.html.SafeUrl) c = goog.html.SafeUrl.unwrap(c);
	        else if ("string" === typeof c) c = goog.html.SafeUrl.sanitize(c).getTypedStringValue();
	        else throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'Attribute "' + b + '" on tag "' + a + '" requires goog.html.SafeUrl, goog.string.Const, or string, value "' + c + '" given.' : "");
	    }
	    c.implementsGoogStringTypedString && (c = c.getTypedStringValue());
	    goog.asserts.assert("string" === typeof c || "number" === typeof c, "String or number value expected, got " + typeof c + " with value: " + c);
	    return b + '="' + goog.string.internal.htmlEscape(String(c)) + '"'
	};
	goog.html.SafeHtml.getStyleValue_ = function (a) {
	    if (!goog.isObject(a)) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'The "style" attribute requires goog.html.SafeStyle or map of style properties, ' + typeof a + " given: " + a : "");
	    a instanceof goog.html.SafeStyle || (a = goog.html.SafeStyle.create(a));
	    return goog.html.SafeStyle.unwrap(a)
	};
	goog.html.SafeHtml.createWithDir = function (a, b, c, d) {
	    b = goog.html.SafeHtml.create(b, c, d);
	    b.dir_ = a;
	    return b
	};
	goog.html.SafeHtml.join = function (a, b) {
	    a = goog.html.SafeHtml.htmlEscape(a);
	    var c = a.getDirection(),
	        d = [],
	        e = function (f) {
	            Array.isArray(f) ? module$contents$goog$array_forEach(f, e) : (f = goog.html.SafeHtml.htmlEscape(f), d.push(goog.html.SafeHtml.unwrap(f)), f = f.getDirection(), c == goog.i18n.bidi.Dir.NEUTRAL ? c = f : f != goog.i18n.bidi.Dir.NEUTRAL && c != f && (c = null));
	        };
	    module$contents$goog$array_forEach(b, e);
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(d.join(goog.html.SafeHtml.unwrap(a)), c)
	};
	goog.html.SafeHtml.concat = function (a) {
	    return goog.html.SafeHtml.join(goog.html.SafeHtml.EMPTY, Array.prototype.slice.call(arguments))
	};
	goog.html.SafeHtml.concatWithDir = function (a, b) {
	    var c = goog.html.SafeHtml.concat(module$contents$goog$array_slice(arguments, 1));
	    c.dir_ = a;
	    return c
	};
	goog.html.SafeHtml.CONSTRUCTOR_TOKEN_PRIVATE_ = {};
	goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse = function (a, b) {
	    var c = goog.html.trustedtypes.getPolicyPrivateDoNotAccessOrElse();
	    a = c ? c.createHTML(a) : a;
	    return new goog.html.SafeHtml(a, b, goog.html.SafeHtml.CONSTRUCTOR_TOKEN_PRIVATE_)
	};
	goog.html.SafeHtml.createSafeHtmlTagSecurityPrivateDoNotAccessOrElse = function (a, b, c) {
	    var d = null;
	    var e = "<" + a + goog.html.SafeHtml.stringifyAttributes(a, b);
	    null == c ? c = [] : Array.isArray(c) || (c = [c]);
	    goog.dom.tags.isVoidTag(a.toLowerCase()) ? (goog.asserts.assert(!c.length, "Void tag <" + a + "> does not allow content."), e += ">") : (d = goog.html.SafeHtml.concat(c), e += ">" + goog.html.SafeHtml.unwrap(d) + "</" + a + ">", d = d.getDirection());
	    (a = b && b.dir) && (d = /^(ltr|rtl|auto)$/i.test(a) ? goog.i18n.bidi.Dir.NEUTRAL : null);
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(e,
	        d)
	};
	goog.html.SafeHtml.stringifyAttributes = function (a, b) {
	    var c = "";
	    if (b)
	        for (var d in b)
	            if (Object.prototype.hasOwnProperty.call(b, d)) {
	                if (!goog.html.SafeHtml.VALID_NAMES_IN_TAG_.test(d)) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'Invalid attribute name "' + d + '".' : "");
	                var e = b[d];
	                null != e && (c += " " + goog.html.SafeHtml.getAttrNameAndValue_(a, d, e));
	            } return c
	};
	goog.html.SafeHtml.combineAttributes = function (a, b, c) {
	    var d = {},
	        e;
	    for (e in a) Object.prototype.hasOwnProperty.call(a, e) && (goog.asserts.assert(e.toLowerCase() == e, "Must be lower case"), d[e] = a[e]);
	    for (e in b) Object.prototype.hasOwnProperty.call(b, e) && (goog.asserts.assert(e.toLowerCase() == e, "Must be lower case"), d[e] = b[e]);
	    if (c)
	        for (e in c)
	            if (Object.prototype.hasOwnProperty.call(c, e)) {
	                var f = e.toLowerCase();
	                if (f in a) throw Error(goog.html.SafeHtml.ENABLE_ERROR_MESSAGES ? 'Cannot override "' + f + '" attribute, got "' +
	                    e + '" with value "' + c[e] + '"' : "");
	                f in b && delete d[f];
	                d[e] = c[e];
	            } return d
	};
	goog.html.SafeHtml.DOCTYPE_HTML = goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse("<!DOCTYPE html>", goog.i18n.bidi.Dir.NEUTRAL);
	goog.html.SafeHtml.EMPTY = new goog.html.SafeHtml(goog.global.trustedTypes && goog.global.trustedTypes.emptyHTML || "", goog.i18n.bidi.Dir.NEUTRAL, goog.html.SafeHtml.CONSTRUCTOR_TOKEN_PRIVATE_);
	goog.html.SafeHtml.BR = goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse("<br>", goog.i18n.bidi.Dir.NEUTRAL);
	goog.html.uncheckedconversions = {};
	goog.html.uncheckedconversions.safeHtmlFromStringKnownToSatisfyTypeContract = function (a, b, c) {
	    goog.asserts.assertString(goog.string.Const.unwrap(a), "must provide justification");
	    goog.asserts.assert(!goog.string.internal.isEmptyOrWhitespace(goog.string.Const.unwrap(a)), "must provide non-empty justification");
	    return goog.html.SafeHtml.createSafeHtmlSecurityPrivateDoNotAccessOrElse(b, c || null)
	};
	goog.html.uncheckedconversions.safeScriptFromStringKnownToSatisfyTypeContract = function (a, b) {
	    goog.asserts.assertString(goog.string.Const.unwrap(a), "must provide justification");
	    goog.asserts.assert(!goog.string.internal.isEmptyOrWhitespace(goog.string.Const.unwrap(a)), "must provide non-empty justification");
	    return module$contents$goog$html$SafeScript_SafeScript.createSafeScriptSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.html.uncheckedconversions.safeStyleFromStringKnownToSatisfyTypeContract = function (a, b) {
	    goog.asserts.assertString(goog.string.Const.unwrap(a), "must provide justification");
	    goog.asserts.assert(!goog.string.internal.isEmptyOrWhitespace(goog.string.Const.unwrap(a)), "must provide non-empty justification");
	    return goog.html.SafeStyle.createSafeStyleSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.html.uncheckedconversions.safeStyleSheetFromStringKnownToSatisfyTypeContract = function (a, b) {
	    goog.asserts.assertString(goog.string.Const.unwrap(a), "must provide justification");
	    goog.asserts.assert(!goog.string.internal.isEmptyOrWhitespace(goog.string.Const.unwrap(a)), "must provide non-empty justification");
	    return module$contents$goog$html$SafeStyleSheet_SafeStyleSheet.createSafeStyleSheetSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.html.uncheckedconversions.safeUrlFromStringKnownToSatisfyTypeContract = function (a, b) {
	    goog.asserts.assertString(goog.string.Const.unwrap(a), "must provide justification");
	    goog.asserts.assert(!goog.string.internal.isEmptyOrWhitespace(goog.string.Const.unwrap(a)), "must provide non-empty justification");
	    return goog.html.SafeUrl.createSafeUrlSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.html.uncheckedconversions.trustedResourceUrlFromStringKnownToSatisfyTypeContract = function (a, b) {
	    goog.asserts.assertString(goog.string.Const.unwrap(a), "must provide justification");
	    goog.asserts.assert(!goog.string.internal.isEmptyOrWhitespace(goog.string.Const.unwrap(a)), "must provide non-empty justification");
	    return goog.html.TrustedResourceUrl.createTrustedResourceUrlSecurityPrivateDoNotAccessOrElse(b)
	};
	goog.dom.safe = {};
	goog.dom.safe.InsertAdjacentHtmlPosition = {
	    AFTERBEGIN: "afterbegin",
	    AFTEREND: "afterend",
	    BEFOREBEGIN: "beforebegin",
	    BEFOREEND: "beforeend"
	};
	goog.dom.safe.insertAdjacentHtml = function (a, b, c) {
	    a.insertAdjacentHTML(b, goog.html.SafeHtml.unwrapTrustedHTML(c));
	};
	goog.dom.safe.SET_INNER_HTML_DISALLOWED_TAGS_ = {
	    MATH: !0,
	    SCRIPT: !0,
	    STYLE: !0,
	    SVG: !0,
	    TEMPLATE: !0
	};
	goog.dom.safe.isInnerHtmlCleanupRecursive_ = goog.functions.cacheReturnValue(function () {
	    if (goog.DEBUG && "undefined" === typeof document) return !1;
	    var a = document.createElement("div"),
	        b = document.createElement("div");
	    b.appendChild(document.createElement("div"));
	    a.appendChild(b);
	    if (goog.DEBUG && !a.firstChild) return !1;
	    b = a.firstChild.firstChild;
	    a.innerHTML = goog.html.SafeHtml.unwrapTrustedHTML(goog.html.SafeHtml.EMPTY);
	    return !b.parentElement
	});
	goog.dom.safe.unsafeSetInnerHtmlDoNotUseOrElse = function (a, b) {
	    if (goog.dom.safe.isInnerHtmlCleanupRecursive_())
	        for (; a.lastChild;) a.removeChild(a.lastChild);
	    a.innerHTML = goog.html.SafeHtml.unwrapTrustedHTML(b);
	};
	goog.dom.safe.setInnerHtml = function (a, b) {
	    if (goog.asserts.ENABLE_ASSERTS && a.tagName && goog.dom.safe.SET_INNER_HTML_DISALLOWED_TAGS_[a.tagName.toUpperCase()]) throw Error("goog.dom.safe.setInnerHtml cannot be used to set content of " + a.tagName + ".");
	    goog.dom.safe.unsafeSetInnerHtmlDoNotUseOrElse(a, b);
	};
	goog.dom.safe.setInnerHtmlFromConstant = function (a, b) {
	    goog.dom.safe.setInnerHtml(a, goog.html.uncheckedconversions.safeHtmlFromStringKnownToSatisfyTypeContract(goog.string.Const.from("Constant HTML to be immediatelly used."), goog.string.Const.unwrap(b)));
	};
	goog.dom.safe.setOuterHtml = function (a, b) {
	    a.outerHTML = goog.html.SafeHtml.unwrapTrustedHTML(b);
	};
	goog.dom.safe.setFormElementAction = function (a, b) {
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    goog.dom.asserts.assertIsHTMLFormElement(a).action = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setButtonFormAction = function (a, b) {
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    goog.dom.asserts.assertIsHTMLButtonElement(a).formAction = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setInputFormAction = function (a, b) {
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    goog.dom.asserts.assertIsHTMLInputElement(a).formAction = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setStyle = function (a, b) {
	    a.style.cssText = goog.html.SafeStyle.unwrap(b);
	};
	goog.dom.safe.documentWrite = function (a, b) {
	    a.write(goog.html.SafeHtml.unwrapTrustedHTML(b));
	};
	goog.dom.safe.setAnchorHref = function (a, b) {
	    goog.dom.asserts.assertIsHTMLAnchorElement(a);
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    a.href = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setImageSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLImageElement(a);
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b, /^data:image\//i.test(b));
	    a.src = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setAudioSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLAudioElement(a);
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b, /^data:audio\//i.test(b));
	    a.src = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setVideoSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLVideoElement(a);
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b, /^data:video\//i.test(b));
	    a.src = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.setEmbedSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLEmbedElement(a);
	    a.src = goog.html.TrustedResourceUrl.unwrapTrustedScriptURL(b);
	};
	goog.dom.safe.setFrameSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLFrameElement(a);
	    a.src = goog.html.TrustedResourceUrl.unwrap(b);
	};
	goog.dom.safe.setIframeSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLIFrameElement(a);
	    a.src = goog.html.TrustedResourceUrl.unwrap(b);
	};
	goog.dom.safe.setIframeSrcdoc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLIFrameElement(a);
	    a.srcdoc = goog.html.SafeHtml.unwrapTrustedHTML(b);
	};
	goog.dom.safe.setLinkHrefAndRel = function (a, b, c) {
	    goog.dom.asserts.assertIsHTMLLinkElement(a);
	    a.rel = c;
	    goog.string.internal.caseInsensitiveContains(c, "stylesheet") ? (goog.asserts.assert(b instanceof goog.html.TrustedResourceUrl, 'URL must be TrustedResourceUrl because "rel" contains "stylesheet"'), a.href = goog.html.TrustedResourceUrl.unwrap(b)) : a.href = b instanceof goog.html.TrustedResourceUrl ? goog.html.TrustedResourceUrl.unwrap(b) : b instanceof goog.html.SafeUrl ? goog.html.SafeUrl.unwrap(b) : goog.html.SafeUrl.unwrap(goog.html.SafeUrl.sanitizeAssertUnchanged(b));
	};
	goog.dom.safe.setObjectData = function (a, b) {
	    goog.dom.asserts.assertIsHTMLObjectElement(a);
	    a.data = goog.html.TrustedResourceUrl.unwrapTrustedScriptURL(b);
	};
	goog.dom.safe.setScriptSrc = function (a, b) {
	    goog.dom.asserts.assertIsHTMLScriptElement(a);
	    a.src = goog.html.TrustedResourceUrl.unwrapTrustedScriptURL(b);
	    goog.dom.safe.setNonceForScriptElement_(a);
	};
	goog.dom.safe.setScriptContent = function (a, b) {
	    goog.dom.asserts.assertIsHTMLScriptElement(a);
	    a.textContent = module$contents$goog$html$SafeScript_SafeScript.unwrapTrustedScript(b);
	    goog.dom.safe.setNonceForScriptElement_(a);
	};
	goog.dom.safe.setNonceForScriptElement_ = function (a) {
	    var b = goog.getScriptNonce(a.ownerDocument && a.ownerDocument.defaultView);
	    b && a.setAttribute("nonce", b);
	};
	goog.dom.safe.setLocationHref = function (a, b) {
	    goog.dom.asserts.assertIsLocation(a);
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    a.href = goog.html.SafeUrl.unwrap(b);
	};
	goog.dom.safe.assignLocation = function (a, b) {
	    goog.dom.asserts.assertIsLocation(a);
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    a.assign(goog.html.SafeUrl.unwrap(b));
	};
	goog.dom.safe.replaceLocation = function (a, b) {
	    b = b instanceof goog.html.SafeUrl ? b : goog.html.SafeUrl.sanitizeAssertUnchanged(b);
	    a.replace(goog.html.SafeUrl.unwrap(b));
	};
	goog.dom.safe.openInWindow = function (a, b, c, d, e) {
	    a = a instanceof goog.html.SafeUrl ? a : goog.html.SafeUrl.sanitizeAssertUnchanged(a);
	    b = b || goog.global;
	    c = c instanceof goog.string.Const ? goog.string.Const.unwrap(c) : c || "";
	    return void 0 !== d || void 0 !== e ? b.open(goog.html.SafeUrl.unwrap(a), c, d, e) : b.open(goog.html.SafeUrl.unwrap(a), c)
	};
	goog.dom.safe.parseFromStringHtml = function (a, b) {
	    return goog.dom.safe.parseFromString(a, b, "text/html")
	};
	goog.dom.safe.parseFromString = function (a, b, c) {
	    return a.parseFromString(goog.html.SafeHtml.unwrapTrustedHTML(b), c)
	};
	goog.dom.safe.createImageFromBlob = function (a) {
	    if (!/^image\/.*/g.test(a.type)) throw Error("goog.dom.safe.createImageFromBlob only accepts MIME type image/.*.");
	    var b = goog.global.URL.createObjectURL(a);
	    a = new goog.global.Image;
	    a.onload = function () {
	        goog.global.URL.revokeObjectURL(b);
	    };
	    goog.dom.safe.setImageSrc(a, goog.html.uncheckedconversions.safeUrlFromStringKnownToSatisfyTypeContract(goog.string.Const.from("Image blob URL."), b));
	    return a
	};
	goog.dom.safe.createContextualFragment = function (a, b) {
	    return a.createContextualFragment(goog.html.SafeHtml.unwrapTrustedHTML(b))
	};
	goog.dom.safe.getStyleNonce = function (a) {
	    if (a && a != goog.global) return goog.dom.safe.getNonce_(a.document, "style");
	    null === goog.dom.safe.cspStyleNonce_ && (goog.dom.safe.cspStyleNonce_ = goog.dom.safe.getNonce_(goog.global.document, "style"));
	    return goog.dom.safe.cspStyleNonce_
	};
	goog.dom.safe.cspStyleNonce_ = null;
	goog.dom.safe.NONCE_PATTERN_ = /^[\w+/_-]+[=]{0,2}$/;
	goog.dom.safe.getNonce_ = function (a, b) {
	    if (!a.querySelector) return "";
	    var c = a.querySelector(b + "[nonce]");
	    c || "style" != b || (c = a.querySelector('link[rel="stylesheet"][nonce]'));
	    return c && (a = c.nonce || c.getAttribute("nonce")) && goog.dom.safe.NONCE_PATTERN_.test(a) ? a : ""
	};
	goog.string.DETECT_DOUBLE_ESCAPING = !1;
	goog.string.FORCE_NON_DOM_HTML_UNESCAPING = !1;
	goog.string.Unicode = {
	    NBSP: "\u00a0"
	};
	goog.string.startsWith = goog.string.internal.startsWith;
	goog.string.endsWith = goog.string.internal.endsWith;
	goog.string.caseInsensitiveStartsWith = goog.string.internal.caseInsensitiveStartsWith;
	goog.string.caseInsensitiveEndsWith = goog.string.internal.caseInsensitiveEndsWith;
	goog.string.caseInsensitiveEquals = goog.string.internal.caseInsensitiveEquals;
	goog.string.subs = function (a, b) {
	    for (var c = a.split("%s"), d = "", e = Array.prototype.slice.call(arguments, 1); e.length && 1 < c.length;) d += c.shift() + e.shift();
	    return d + c.join("%s")
	};
	goog.string.collapseWhitespace = function (a) {
	    return a.replace(/[\s\xa0]+/g, " ").replace(/^\s+|\s+$/g, "")
	};
	goog.string.isEmptyOrWhitespace = goog.string.internal.isEmptyOrWhitespace;
	goog.string.isEmptyString = function (a) {
	    return 0 == a.length
	};
	goog.string.isEmpty = goog.string.isEmptyOrWhitespace;
	goog.string.isEmptyOrWhitespaceSafe = function (a) {
	    return goog.string.isEmptyOrWhitespace(goog.string.makeSafe(a))
	};
	goog.string.isEmptySafe = goog.string.isEmptyOrWhitespaceSafe;
	goog.string.isBreakingWhitespace = function (a) {
	    return !/[^\t\n\r ]/.test(a)
	};
	goog.string.isAlpha = function (a) {
	    return !/[^a-zA-Z]/.test(a)
	};
	goog.string.isNumeric = function (a) {
	    return !/[^0-9]/.test(a)
	};
	goog.string.isAlphaNumeric = function (a) {
	    return !/[^a-zA-Z0-9]/.test(a)
	};
	goog.string.isSpace = function (a) {
	    return " " == a
	};
	goog.string.isUnicodeChar = function (a) {
	    return 1 == a.length && " " <= a && "~" >= a || "\u0080" <= a && "\ufffd" >= a
	};
	goog.string.stripNewlines = function (a) {
	    return a.replace(/(\r\n|\r|\n)+/g, " ")
	};
	goog.string.canonicalizeNewlines = function (a) {
	    return a.replace(/(\r\n|\r|\n)/g, "\n")
	};
	goog.string.normalizeWhitespace = function (a) {
	    return a.replace(/\xa0|\s/g, " ")
	};
	goog.string.normalizeSpaces = function (a) {
	    return a.replace(/\xa0|[ \t]+/g, " ")
	};
	goog.string.collapseBreakingSpaces = function (a) {
	    return a.replace(/[\t\r\n ]+/g, " ").replace(/^[\t\r\n ]+|[\t\r\n ]+$/g, "")
	};
	goog.string.trim = goog.string.internal.trim;
	goog.string.trimLeft = function (a) {
	    return a.replace(/^[\s\xa0]+/, "")
	};
	goog.string.trimRight = function (a) {
	    return a.replace(/[\s\xa0]+$/, "")
	};
	goog.string.caseInsensitiveCompare = goog.string.internal.caseInsensitiveCompare;
	goog.string.numberAwareCompare_ = function (a, b, c) {
	    if (a == b) return 0;
	    if (!a) return -1;
	    if (!b) return 1;
	    for (var d = a.toLowerCase().match(c), e = b.toLowerCase().match(c), f = Math.min(d.length, e.length), g = 0; g < f; g++) {
	        c = d[g];
	        var h = e[g];
	        if (c != h) return a = parseInt(c, 10), !isNaN(a) && (b = parseInt(h, 10), !isNaN(b) && a - b) ? a - b : c < h ? -1 : 1
	    }
	    return d.length != e.length ? d.length - e.length : a < b ? -1 : 1
	};
	goog.string.intAwareCompare = function (a, b) {
	    return goog.string.numberAwareCompare_(a, b, /\d+|\D+/g)
	};
	goog.string.floatAwareCompare = function (a, b) {
	    return goog.string.numberAwareCompare_(a, b, /\d+|\.\d+|\D+/g)
	};
	goog.string.numerateCompare = goog.string.floatAwareCompare;
	goog.string.urlEncode = function (a) {
	    return encodeURIComponent(String(a))
	};
	goog.string.urlDecode = function (a) {
	    return decodeURIComponent(a.replace(/\+/g, " "))
	};
	goog.string.newLineToBr = goog.string.internal.newLineToBr;
	goog.string.htmlEscape = function (a, b) {
	    a = goog.string.internal.htmlEscape(a, b);
	    goog.string.DETECT_DOUBLE_ESCAPING && (a = a.replace(goog.string.E_RE_, "&#101;"));
	    return a
	};
	goog.string.E_RE_ = /e/g;
	goog.string.unescapeEntities = function (a) {
	    return goog.string.contains(a, "&") ? !goog.string.FORCE_NON_DOM_HTML_UNESCAPING && "document" in goog.global ? goog.string.unescapeEntitiesUsingDom_(a) : goog.string.unescapePureXmlEntities_(a) : a
	};
	goog.string.unescapeEntitiesWithDocument = function (a, b) {
	    return goog.string.contains(a, "&") ? goog.string.unescapeEntitiesUsingDom_(a, b) : a
	};
	goog.string.unescapeEntitiesUsingDom_ = function (a, b) {
	    var c = {
	        "&amp;": "&",
	        "&lt;": "<",
	        "&gt;": ">",
	        "&quot;": '"'
	    };
	    var d = b ? b.createElement("div") : goog.global.document.createElement("div");
	    return a.replace(goog.string.HTML_ENTITY_PATTERN_, function (e, f) {
	        var g = c[e];
	        if (g) return g;
	        "#" == f.charAt(0) && (f = Number("0" + f.substr(1)), isNaN(f) || (g = String.fromCharCode(f)));
	        g || (goog.dom.safe.setInnerHtml(d, goog.html.uncheckedconversions.safeHtmlFromStringKnownToSatisfyTypeContract(goog.string.Const.from("Single HTML entity."),
	            e + " ")), g = d.firstChild.nodeValue.slice(0, -1));
	        return c[e] = g
	    })
	};
	goog.string.unescapePureXmlEntities_ = function (a) {
	    return a.replace(/&([^;]+);/g, function (b, c) {
	        switch (c) {
	            case "amp":
	                return "&";
	            case "lt":
	                return "<";
	            case "gt":
	                return ">";
	            case "quot":
	                return '"';
	            default:
	                return "#" != c.charAt(0) || (c = Number("0" + c.substr(1)), isNaN(c)) ? b : String.fromCharCode(c)
	        }
	    })
	};
	goog.string.HTML_ENTITY_PATTERN_ = /&([^;\s<&]+);?/g;
	goog.string.whitespaceEscape = function (a, b) {
	    return goog.string.newLineToBr(a.replace(/  /g, " &#160;"), b)
	};
	goog.string.preserveSpaces = function (a) {
	    return a.replace(/(^|[\n ]) /g, "$1" + goog.string.Unicode.NBSP)
	};
	goog.string.stripQuotes = function (a, b) {
	    for (var c = b.length, d = 0; d < c; d++) {
	        var e = 1 == c ? b : b.charAt(d);
	        if (a.charAt(0) == e && a.charAt(a.length - 1) == e) return a.substring(1, a.length - 1)
	    }
	    return a
	};
	goog.string.truncate = function (a, b, c) {
	    c && (a = goog.string.unescapeEntities(a));
	    a.length > b && (a = a.substring(0, b - 3) + "...");
	    c && (a = goog.string.htmlEscape(a));
	    return a
	};
	goog.string.truncateMiddle = function (a, b, c, d) {
	    c && (a = goog.string.unescapeEntities(a));
	    d && a.length > b ? (d > b && (d = b), a = a.substring(0, b - d) + "..." + a.substring(a.length - d)) : a.length > b && (d = Math.floor(b / 2), a = a.substring(0, d + b % 2) + "..." + a.substring(a.length - d));
	    c && (a = goog.string.htmlEscape(a));
	    return a
	};
	goog.string.specialEscapeChars_ = {
	    "\x00": "\\0",
	    "\b": "\\b",
	    "\f": "\\f",
	    "\n": "\\n",
	    "\r": "\\r",
	    "\t": "\\t",
	    "\x0B": "\\x0B",
	    '"': '\\"',
	    "\\": "\\\\",
	    "<": "\\u003C"
	};
	goog.string.jsEscapeCache_ = {
	    "'": "\\'"
	};
	goog.string.quote = function (a) {
	    a = String(a);
	    for (var b = ['"'], c = 0; c < a.length; c++) {
	        var d = a.charAt(c),
	            e = d.charCodeAt(0);
	        b[c + 1] = goog.string.specialEscapeChars_[d] || (31 < e && 127 > e ? d : goog.string.escapeChar(d));
	    }
	    b.push('"');
	    return b.join("")
	};
	goog.string.escapeString = function (a) {
	    for (var b = [], c = 0; c < a.length; c++) b[c] = goog.string.escapeChar(a.charAt(c));
	    return b.join("")
	};
	goog.string.escapeChar = function (a) {
	    if (a in goog.string.jsEscapeCache_) return goog.string.jsEscapeCache_[a];
	    if (a in goog.string.specialEscapeChars_) return goog.string.jsEscapeCache_[a] = goog.string.specialEscapeChars_[a];
	    var b = a.charCodeAt(0);
	    if (31 < b && 127 > b) var c = a;
	    else {
	        if (256 > b) {
	            if (c = "\\x", 16 > b || 256 < b) c += "0";
	        } else c = "\\u", 4096 > b && (c += "0");
	        c += b.toString(16).toUpperCase();
	    }
	    return goog.string.jsEscapeCache_[a] = c
	};
	goog.string.contains = goog.string.internal.contains;
	goog.string.caseInsensitiveContains = goog.string.internal.caseInsensitiveContains;
	goog.string.countOf = function (a, b) {
	    return a && b ? a.split(b).length - 1 : 0
	};
	goog.string.removeAt = function (a, b, c) {
	    var d = a;
	    0 <= b && b < a.length && 0 < c && (d = a.substr(0, b) + a.substr(b + c, a.length - b - c));
	    return d
	};
	goog.string.remove = function (a, b) {
	    return a.replace(b, "")
	};
	goog.string.removeAll = function (a, b) {
	    b = new RegExp(goog.string.regExpEscape(b), "g");
	    return a.replace(b, "")
	};
	goog.string.replaceAll = function (a, b, c) {
	    b = new RegExp(goog.string.regExpEscape(b), "g");
	    return a.replace(b, c.replace(/\$/g, "$$$$"))
	};
	goog.string.regExpEscape = function (a) {
	    return String(a).replace(/([-()\[\]{}+?*.$\^|,:#<!\\])/g, "\\$1").replace(/\x08/g, "\\x08")
	};
	goog.string.repeat = String.prototype.repeat ? function (a, b) {
	    return a.repeat(b)
	} : function (a, b) {
	    return Array(b + 1).join(a)
	};
	goog.string.padNumber = function (a, b, c) {
	    a = void 0 !== c ? a.toFixed(c) : String(a);
	    c = a.indexOf("."); - 1 == c && (c = a.length);
	    return goog.string.repeat("0", Math.max(0, b - c)) + a
	};
	goog.string.makeSafe = function (a) {
	    return null == a ? "" : String(a)
	};
	goog.string.buildString = function (a) {
	    return Array.prototype.join.call(arguments, "")
	};
	goog.string.getRandomString = function () {
	    return Math.floor(2147483648 * Math.random()).toString(36) + Math.abs(Math.floor(2147483648 * Math.random()) ^ goog.now()).toString(36)
	};
	goog.string.compareVersions = goog.string.internal.compareVersions;
	goog.string.hashCode = function (a) {
	    for (var b = 0, c = 0; c < a.length; ++c) b = 31 * b + a.charCodeAt(c) >>> 0;
	    return b
	};
	goog.string.uniqueStringCounter_ = 2147483648 * Math.random() | 0;
	goog.string.createUniqueString = function () {
	    return "goog_" + goog.string.uniqueStringCounter_++
	};
	goog.string.toNumber = function (a) {
	    var b = Number(a);
	    return 0 == b && goog.string.isEmptyOrWhitespace(a) ? NaN : b
	};
	goog.string.isLowerCamelCase = function (a) {
	    return /^[a-z]+([A-Z][a-z]*)*$/.test(a)
	};
	goog.string.isUpperCamelCase = function (a) {
	    return /^([A-Z][a-z]*)+$/.test(a)
	};
	goog.string.toCamelCase = function (a) {
	    return String(a).replace(/\-([a-z])/g, function (b, c) {
	        return c.toUpperCase()
	    })
	};
	goog.string.toSelectorCase = function (a) {
	    return String(a).replace(/([A-Z])/g, "-$1").toLowerCase()
	};
	goog.string.toTitleCase = function (a, b) {
	    b = "string" === typeof b ? goog.string.regExpEscape(b) : "\\s";
	    return a.replace(new RegExp("(^" + (b ? "|[" + b + "]+" : "") + ")([a-z])", "g"), function (c, d, e) {
	        return d + e.toUpperCase()
	    })
	};
	goog.string.capitalize = function (a) {
	    return String(a.charAt(0)).toUpperCase() + String(a.substr(1)).toLowerCase()
	};
	goog.string.parseInt = function (a) {
	    isFinite(a) && (a = String(a));
	    return "string" === typeof a ? /^\s*-?0x/i.test(a) ? parseInt(a, 16) : parseInt(a, 10) : NaN
	};
	goog.string.splitLimit = function (a, b, c) {
	    a = a.split(b);
	    for (var d = []; 0 < c && a.length;) d.push(a.shift()), c--;
	    a.length && d.push(a.join(b));
	    return d
	};
	goog.string.lastComponent = function (a, b) {
	    if (b) "string" == typeof b && (b = [b]);
	    else return a;
	    for (var c = -1, d = 0; d < b.length; d++)
	        if ("" != b[d]) {
	            var e = a.lastIndexOf(b[d]);
	            e > c && (c = e);
	        } return -1 == c ? a : a.slice(c + 1)
	};
	goog.string.editDistance = function (a, b) {
	    var c = [],
	        d = [];
	    if (a == b) return 0;
	    if (!a.length || !b.length) return Math.max(a.length, b.length);
	    for (var e = 0; e < b.length + 1; e++) c[e] = e;
	    for (e = 0; e < a.length; e++) {
	        d[0] = e + 1;
	        for (var f = 0; f < b.length; f++) d[f + 1] = Math.min(d[f] + 1, c[f + 1] + 1, c[f] + Number(a[e] != b[f]));
	        for (f = 0; f < c.length; f++) c[f] = d[f];
	    }
	    return d[b.length]
	};
	jspb.utils = {};
	var module$contents$jspb$utils_split64Low = 0,
	    module$contents$jspb$utils_split64High = 0;

	function module$contents$jspb$utils_splitUint64(a) {
	    var b = a >>> 0;
	    a = Math.floor((a - b) / 4294967296) >>> 0;
	    module$contents$jspb$utils_split64Low = b;
	    module$contents$jspb$utils_split64High = a;
	}

	function module$contents$jspb$utils_splitInt64(a) {
	    var b = 0 > a;
	    a = Math.abs(a);
	    var c = a >>> 0;
	    a = Math.floor((a - c) / 4294967296);
	    a >>>= 0;
	    b && (a = ~a >>> 0, c = (~c >>> 0) + 1, 4294967295 < c && (c = 0, a++, 4294967295 < a && (a = 0)));
	    module$contents$jspb$utils_split64Low = c;
	    module$contents$jspb$utils_split64High = a;
	}

	function module$contents$jspb$utils_splitZigzag64(a) {
	    var b = 0 > a;
	    a = 2 * Math.abs(a);
	    module$contents$jspb$utils_splitUint64(a);
	    a = module$contents$jspb$utils_split64Low;
	    var c = module$contents$jspb$utils_split64High;
	    b && (0 == a ? 0 == c ? c = a = 4294967295 : (c--, a = 4294967295) : a--);
	    module$contents$jspb$utils_split64Low = a;
	    module$contents$jspb$utils_split64High = c;
	}

	function module$contents$jspb$utils_splitFloat32(a) {
	    var b = 0 > a ? 1 : 0;
	    a = b ? -a : a;
	    if (0 === a) 0 < 1 / a ? module$contents$jspb$utils_split64Low = module$contents$jspb$utils_split64High = 0 : (module$contents$jspb$utils_split64High = 0, module$contents$jspb$utils_split64Low = 2147483648);
	    else if (isNaN(a)) module$contents$jspb$utils_split64High = 0, module$contents$jspb$utils_split64Low = 2147483647;
	    else if (3.4028234663852886E38 < a) module$contents$jspb$utils_split64High = 0, module$contents$jspb$utils_split64Low = (b << 31 | 2139095040) >>> 0;
	    else if (1.1754943508222875E-38 > a) a = Math.round(a / Math.pow(2, -149)), module$contents$jspb$utils_split64High = 0, module$contents$jspb$utils_split64Low = (b << 31 | a) >>> 0;
	    else {
	        var c = Math.floor(Math.log(a) / Math.LN2);
	        a *= Math.pow(2, -c);
	        a = Math.round(8388608 * a) & 8388607;
	        module$contents$jspb$utils_split64High = 0;
	        module$contents$jspb$utils_split64Low = (b << 31 | c + 127 << 23 | a) >>> 0;
	    }
	}

	function module$contents$jspb$utils_splitFloat64(a) {
	    var b = 0 > a ? 1 : 0;
	    a = b ? -a : a;
	    if (0 === a) module$contents$jspb$utils_split64High = 0 < 1 / a ? 0 : 2147483648, module$contents$jspb$utils_split64Low = 0;
	    else if (isNaN(a)) module$contents$jspb$utils_split64High = 2147483647, module$contents$jspb$utils_split64Low = 4294967295;
	    else if (1.7976931348623157E308 < a) module$contents$jspb$utils_split64High = (b << 31 | 2146435072) >>> 0, module$contents$jspb$utils_split64Low = 0;
	    else if (2.2250738585072014E-308 > a) a /= Math.pow(2, -1074), module$contents$jspb$utils_split64High =
	        (b << 31 | a / 4294967296) >>> 0, module$contents$jspb$utils_split64Low = a >>> 0;
	    else {
	        var c = a,
	            d = 0;
	        if (2 <= c)
	            for (; 2 <= c && 1023 > d;) d++, c /= 2;
	        else
	            for (; 1 > c && -1022 < d;) c *= 2, d--;
	        a *= Math.pow(2, -d);
	        module$contents$jspb$utils_split64High = (b << 31 | d + 1023 << 20 | 1048576 * a & 1048575) >>> 0;
	        module$contents$jspb$utils_split64Low = 4503599627370496 * a >>> 0;
	    }
	}

	function module$contents$jspb$utils_splitHash64(a) {
	    var b = a.charCodeAt(4),
	        c = a.charCodeAt(5),
	        d = a.charCodeAt(6),
	        e = a.charCodeAt(7);
	    module$contents$jspb$utils_split64Low = a.charCodeAt(0) + (a.charCodeAt(1) << 8) + (a.charCodeAt(2) << 16) + (a.charCodeAt(3) << 24) >>> 0;
	    module$contents$jspb$utils_split64High = b + (c << 8) + (d << 16) + (e << 24) >>> 0;
	}

	function module$contents$jspb$utils_joinUint64(a, b) {
	    return 4294967296 * b + (a >>> 0)
	}

	function module$contents$jspb$utils_joinInt64(a, b) {
	    var c = b & 2147483648;
	    c && (a = ~a + 1 >>> 0, b = ~b >>> 0, 0 == a && (b = b + 1 >>> 0));
	    a = module$contents$jspb$utils_joinUint64(a, b);
	    return c ? -a : a
	}

	function module$contents$jspb$utils_toZigzag64(a, b, c) {
	    var d = b >> 31;
	    return c(a << 1 ^ d, (b << 1 | a >>> 31) ^ d)
	}

	function module$contents$jspb$utils_joinZigzag64(a, b) {
	    return module$contents$jspb$utils_fromZigzag64(a, b, module$contents$jspb$utils_joinInt64)
	}

	function module$contents$jspb$utils_fromZigzag64(a, b, c) {
	    var d = -(a & 1);
	    return c((a >>> 1 | b << 31) ^ d, b >>> 1 ^ d)
	}

	function module$contents$jspb$utils_joinFloat32(a) {
	    var b = 2 * (a >> 31) + 1,
	        c = a >>> 23 & 255;
	    a &= 8388607;
	    return 255 == c ? a ? NaN : Infinity * b : 0 == c ? b * Math.pow(2, -149) * a : b * Math.pow(2, c - 150) * (a + Math.pow(2, 23))
	}

	function module$contents$jspb$utils_joinFloat64(a, b) {
	    var c = 2 * (b >> 31) + 1,
	        d = b >>> 20 & 2047;
	    a = 4294967296 * (b & 1048575) + a;
	    return 2047 == d ? a ? NaN : Infinity * c : 0 == d ? c * Math.pow(2, -1074) * a : c * Math.pow(2, d - 1075) * (a + 4503599627370496)
	}

	function module$contents$jspb$utils_joinHash64(a, b) {
	    return String.fromCharCode(a >>> 0 & 255, a >>> 8 & 255, a >>> 16 & 255, a >>> 24 & 255, b >>> 0 & 255, b >>> 8 & 255, b >>> 16 & 255, b >>> 24 & 255)
	}

	function module$contents$jspb$utils_joinUnsignedDecimalString(a, b) {
	    function c(e, f) {
	        e = e ? String(e) : "";
	        return f ? "0000000".slice(e.length) + e : e
	    }
	    if (2097151 >= b) return "" + (4294967296 * b + a);
	    var d = (a >>> 24 | b << 8) >>> 0 & 16777215;
	    b = b >> 16 & 65535;
	    a = (a & 16777215) + 6777216 * d + 6710656 * b;
	    d += 8147497 * b;
	    b *= 2;
	    1E7 <= a && (d += Math.floor(a / 1E7), a %= 1E7);
	    1E7 <= d && (b += Math.floor(d / 1E7), d %= 1E7);
	    return c(b, 0) + c(d, b) + c(a, 1)
	}

	function module$contents$jspb$utils_joinSignedDecimalString(a, b) {
	    var c = b & 2147483648;
	    c && (a = ~a + 1 >>> 0, b = ~b + (0 == a ? 1 : 0) >>> 0);
	    a = module$contents$jspb$utils_joinUnsignedDecimalString(a, b);
	    return c ? "-" + a : a
	}

	function module$contents$jspb$utils_hash64ToDecimalString(a, b) {
	    module$contents$jspb$utils_splitHash64(a);
	    a = module$contents$jspb$utils_split64Low;
	    var c = module$contents$jspb$utils_split64High;
	    return b ? module$contents$jspb$utils_joinSignedDecimalString(a, c) : module$contents$jspb$utils_joinUnsignedDecimalString(a, c)
	}

	function module$contents$jspb$utils_decimalStringToHash64(a) {
	    function b(g, h) {
	        for (var k = 0; 8 > k && (1 !== g || 0 < h); k++) h = g * e[k] + h, e[k] = h & 255, h >>>= 8;
	    }

	    function c() {
	        for (var g = 0; 8 > g; g++) e[g] = ~e[g] & 255;
	    }(0, goog.asserts.assert)(0 < a.length);
	    var d = !1;
	    "-" === a[0] && (d = !0, a = a.slice(1));
	    for (var e = [0, 0, 0, 0, 0, 0, 0, 0], f = 0; f < a.length; f++) b(10, a.charCodeAt(f) - 48);
	    d && (c(), b(1, 1));
	    return goog.crypt.byteArrayToString(e)
	}

	function module$contents$jspb$utils_splitDecimalString(a) {
	    module$contents$jspb$utils_splitHash64(module$contents$jspb$utils_decimalStringToHash64(a));
	}

	function module$contents$jspb$utils_toHexDigit_(a) {
	    return String.fromCharCode(10 > a ? 48 + a : 87 + a)
	}

	function module$contents$jspb$utils_fromHexCharCode_(a) {
	    return 97 <= a ? a - 97 + 10 : a - 48
	}

	function module$contents$jspb$utils_countFixedFields_(a, b, c, d, e) {
	    var f = 0;
	    if (128 > d)
	        for (; b < c && a[b++] == d;) f++, b += e;
	    else
	        for (; b < c;) {
	            for (var g = d; 128 < g;) {
	                if (a[b++] != (g & 127 | 128)) return f;
	                g >>= 7;
	            }
	            if (a[b++] != g) break;
	            f++;
	            b += e;
	        }
	    return f
	}

	function module$contents$jspb$utils_byteSourceToUint8Array(a) {
	    if (a.constructor === Uint8Array) return a;
	    if (a.constructor === ArrayBuffer) return new Uint8Array(a);
	    if (a.constructor === Array) return new Uint8Array(a);
	    if (a.constructor === String) return goog.crypt.base64.decodeStringToUint8Array(a);
	    if (a instanceof Uint8Array) return new Uint8Array(a.buffer, a.byteOffset, a.byteLength);
	    (0, goog.asserts.fail)("Type not convertible to Uint8Array.");
	    return new Uint8Array(0)
	}
	jspb.utils.byteSourceToUint8Array = module$contents$jspb$utils_byteSourceToUint8Array;
	jspb.utils.countDelimitedFields = function (a, b, c, d) {
	    var e = 0;
	    for (d = 8 * d + module$contents$jspb$BinaryConstants_WireType.DELIMITED; b < c;) {
	        for (var f = d; 128 < f;) {
	            if (a[b++] != (f & 127 | 128)) return e;
	            f >>= 7;
	        }
	        if (a[b++] != f) break;
	        e++;
	        for (var g = 0, h = 1; f = a[b++], g += (f & 127) * h, h *= 128, 0 != (f & 128););
	        b += g;
	    }
	    return e
	};
	jspb.utils.countFixed32Fields = function (a, b, c, d) {
	    return module$contents$jspb$utils_countFixedFields_(a, b, c, 8 * d + module$contents$jspb$BinaryConstants_WireType.FIXED32, 4)
	};
	jspb.utils.countFixed64Fields = function (a, b, c, d) {
	    return module$contents$jspb$utils_countFixedFields_(a, b, c, 8 * d + module$contents$jspb$BinaryConstants_WireType.FIXED64, 8)
	};
	jspb.utils.countVarintFields = function (a, b, c, d) {
	    var e = 0;
	    d = 8 * d + module$contents$jspb$BinaryConstants_WireType.VARINT;
	    if (128 > d)
	        for (; b < c && a[b++] == d;)
	            for (e++;;) {
	                var f = a[b++];
	                if (0 == (f & 128)) break
	            } else
	                for (; b < c;) {
	                    for (f = d; 128 < f;) {
	                        if (a[b] != (f & 127 | 128)) return e;
	                        b++;
	                        f >>= 7;
	                    }
	                    if (a[b++] != f) break;
	                    for (e++; f = a[b++], 0 != (f & 128););
	                }
	    return e
	};
	jspb.utils.countVarints = function (a, b, c) {
	    for (var d = 0, e = b; e < c; e++) d += a[e] >> 7;
	    return c - b - d
	};
	jspb.utils.debugBytesToTextFormat = function (a) {
	    var b = '"';
	    if (a) {
	        a = module$contents$jspb$utils_byteSourceToUint8Array(a);
	        for (var c = 0; c < a.length; c++) b += "\\x", 16 > a[c] && (b += "0"), b += a[c].toString(16);
	    }
	    return b + '"'
	};
	jspb.utils.debugScalarToTextFormat = function (a) {
	    return "string" === typeof a ? goog.string.quote(a) : a.toString()
	};
	jspb.utils.decimalStringToHash64 = module$contents$jspb$utils_decimalStringToHash64;
	jspb.utils.DIGITS = "0123456789abcdef".split("");
	jspb.utils.fromZigzag64 = module$contents$jspb$utils_fromZigzag64;
	jspb.utils.hash64ArrayToDecimalStrings = function (a, b) {
	    for (var c = Array(a.length), d = 0; d < a.length; d++) c[d] = module$contents$jspb$utils_hash64ToDecimalString(a[d], b);
	    return c
	};
	jspb.utils.hash64ToDecimalString = module$contents$jspb$utils_hash64ToDecimalString;
	jspb.utils.hash64ToHexString = function (a) {
	    var b = Array(18);
	    b[0] = "0";
	    b[1] = "x";
	    for (var c = 0; 8 > c; c++) {
	        var d = a.charCodeAt(7 - c);
	        b[2 * c + 2] = module$contents$jspb$utils_toHexDigit_(d >> 4);
	        b[2 * c + 3] = module$contents$jspb$utils_toHexDigit_(d & 15);
	    }
	    return b.join("")
	};
	jspb.utils.hash64ToNumber = function (a, b) {
	    module$contents$jspb$utils_splitHash64(a);
	    a = module$contents$jspb$utils_split64Low;
	    var c = module$contents$jspb$utils_split64High;
	    return b ? module$contents$jspb$utils_joinInt64(a, c) : module$contents$jspb$utils_joinUint64(a, c)
	};
	jspb.utils.hexStringToHash64 = function (a) {
	    a = a.toLowerCase();
	    (0, goog.asserts.assert)(18 == a.length);
	    (0, goog.asserts.assert)("0" == a[0]);
	    (0, goog.asserts.assert)("x" == a[1]);
	    for (var b = "", c = 0; 8 > c; c++) b = String.fromCharCode(16 * module$contents$jspb$utils_fromHexCharCode_(a.charCodeAt(2 * c + 2)) + module$contents$jspb$utils_fromHexCharCode_(a.charCodeAt(2 * c + 3))) + b;
	    return b
	};
	jspb.utils.joinFloat64 = module$contents$jspb$utils_joinFloat64;
	jspb.utils.joinFloat32 = module$contents$jspb$utils_joinFloat32;
	jspb.utils.joinHash64 = module$contents$jspb$utils_joinHash64;
	jspb.utils.joinInt64 = module$contents$jspb$utils_joinInt64;
	jspb.utils.joinSignedDecimalString = module$contents$jspb$utils_joinSignedDecimalString;
	jspb.utils.joinUint64 = module$contents$jspb$utils_joinUint64;
	jspb.utils.joinUnsignedDecimalString = module$contents$jspb$utils_joinUnsignedDecimalString;
	jspb.utils.joinZigzag64 = module$contents$jspb$utils_joinZigzag64;
	jspb.utils.numberToHash64 = function (a) {
	    module$contents$jspb$utils_splitInt64(a);
	    return module$contents$jspb$utils_joinHash64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High)
	};
	jspb.utils.splitDecimalString = module$contents$jspb$utils_splitDecimalString;
	jspb.utils.splitHash64 = module$contents$jspb$utils_splitHash64;
	jspb.utils.splitFloat64 = module$contents$jspb$utils_splitFloat64;
	jspb.utils.splitFloat32 = module$contents$jspb$utils_splitFloat32;
	jspb.utils.splitZigzag64 = module$contents$jspb$utils_splitZigzag64;
	jspb.utils.splitInt64 = module$contents$jspb$utils_splitInt64;
	jspb.utils.splitUint64 = module$contents$jspb$utils_splitUint64;
	jspb.utils.getSplit64Low = function () {
	    return module$contents$jspb$utils_split64Low
	};
	jspb.utils.getSplit64High = function () {
	    return module$contents$jspb$utils_split64High
	};
	jspb.utils.stringToByteArray = function (a) {
	    for (var b = new Uint8Array(a.length), c = 0; c < a.length; c++) {
	        var d = a.charCodeAt(c);
	        if (255 < d) throw Error("Conversion error: string contains codepoint outside of byte range");
	        b[c] = d;
	    }
	    return b
	};
	jspb.utils.toZigzag64 = module$contents$jspb$utils_toZigzag64;
	var module$contents$jspb$BinaryDecoder_BinaryDecoder = function (a, b, c) {
	    this.bytes_ = null;
	    this.cursor_ = this.end_ = this.start_ = 0;
	    this.error_ = !1;
	    a && this.setBlock(a, b, c);
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.alloc = function (a, b, c) {
	    if (module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_.length) {
	        var d = module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_.pop();
	        a && d.setBlock(a, b, c);
	        return d
	    }
	    return new module$contents$jspb$BinaryDecoder_BinaryDecoder(a, b, c)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.free = function () {
	    this.clear();
	    100 > module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_.length && module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_.push(this);
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.clone = function () {
	    return module$contents$jspb$BinaryDecoder_BinaryDecoder.alloc(this.bytes_, this.start_, this.end_ - this.start_)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.clear = function () {
	    this.bytes_ = null;
	    this.cursor_ = this.end_ = this.start_ = 0;
	    this.error_ = !1;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.getBuffer = function () {
	    return this.bytes_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.setBlock = function (a, b, c) {
	    this.bytes_ = module$contents$jspb$utils_byteSourceToUint8Array(a);
	    this.start_ = void 0 !== b ? b : 0;
	    this.end_ = void 0 !== c ? this.start_ + c : this.bytes_.length;
	    this.cursor_ = this.start_;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.getEnd = function () {
	    return this.end_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.setEnd = function (a) {
	    this.end_ = a;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.reset = function () {
	    this.cursor_ = this.start_;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.getCursor = function () {
	    return this.cursor_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.setCursor = function (a) {
	    this.cursor_ = a;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.advance = function (a) {
	    this.cursor_ += a;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.atEnd = function () {
	    return this.cursor_ == this.end_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.pastEnd = function () {
	    return this.cursor_ > this.end_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.getError = function () {
	    return this.error_ || 0 > this.cursor_ || this.cursor_ > this.end_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSplitVarint64 = function (a) {
	    for (var b = 128, c = 0, d = 0, e = 0; 4 > e && 128 <= b; e++) b = this.bytes_[this.cursor_++], c |= (b & 127) << 7 * e;
	    128 <= b && (b = this.bytes_[this.cursor_++], c |= (b & 127) << 28, d |= (b & 127) >> 4);
	    if (128 <= b)
	        for (e = 0; 5 > e && 128 <= b; e++) b = this.bytes_[this.cursor_++], d |= (b & 127) << 7 * e + 3;
	    if (128 > b) return a(c >>> 0, d >>> 0);
	    goog.asserts.fail("Failed to read varint, encoding is invalid.");
	    this.error_ = !0;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSplitZigzagVarint64 = function (a) {
	    return this.readSplitVarint64(function (b, c) {
	        return module$contents$jspb$utils_fromZigzag64(b, c, a)
	    })
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSplitFixed64 = function (a) {
	    var b = this.bytes_,
	        c = this.cursor_;
	    this.cursor_ += 8;
	    for (var d = 0, e = 0, f = c + 7; f >= c; f--) d = d << 8 | b[f], e = e << 8 | b[f + 4];
	    return a(d, e)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.skipVarint = function () {
	    for (; this.bytes_[this.cursor_] & 128;) this.cursor_++;
	    this.cursor_++;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.unskipVarint = function (a) {
	    for (; 128 < a;) this.cursor_--, a >>>= 7;
	    this.cursor_--;
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUnsignedVarint32 = function () {
	    var a = this.bytes_;
	    var b = a[this.cursor_ + 0];
	    var c = b & 127;
	    if (128 > b) return this.cursor_ += 1, goog.asserts.assert(this.cursor_ <= this.end_), c;
	    b = a[this.cursor_ + 1];
	    c |= (b & 127) << 7;
	    if (128 > b) return this.cursor_ += 2, goog.asserts.assert(this.cursor_ <= this.end_), c;
	    b = a[this.cursor_ + 2];
	    c |= (b & 127) << 14;
	    if (128 > b) return this.cursor_ += 3, goog.asserts.assert(this.cursor_ <= this.end_), c;
	    b = a[this.cursor_ + 3];
	    c |= (b & 127) << 21;
	    if (128 > b) return this.cursor_ +=
	        4, goog.asserts.assert(this.cursor_ <= this.end_), c;
	    b = a[this.cursor_ + 4];
	    c |= (b & 15) << 28;
	    if (128 > b) return this.cursor_ += 5, goog.asserts.assert(this.cursor_ <= this.end_), c >>> 0;
	    this.cursor_ += 5;
	    128 <= a[this.cursor_++] && 128 <= a[this.cursor_++] && 128 <= a[this.cursor_++] && 128 <= a[this.cursor_++] && 128 <= a[this.cursor_++] && goog.asserts.assert(!1);
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return c
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSignedVarint32 = function () {
	    return this.readUnsignedVarint32()
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUnsignedVarint32String = function () {
	    return this.readUnsignedVarint32().toString()
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSignedVarint32String = function () {
	    return this.readSignedVarint32().toString()
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readZigzagVarint32 = function () {
	    var a = this.readUnsignedVarint32();
	    return a >>> 1 ^ -(a & 1)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUnsignedVarint64 = function () {
	    return this.readSplitVarint64(module$contents$jspb$utils_joinUint64)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUnsignedVarint64String = function () {
	    return this.readSplitVarint64(module$contents$jspb$utils_joinUnsignedDecimalString)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSignedVarint64 = function () {
	    return this.readSplitVarint64(module$contents$jspb$utils_joinInt64)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readSignedVarint64String = function () {
	    return this.readSplitVarint64(module$contents$jspb$utils_joinSignedDecimalString)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readZigzagVarint64 = function () {
	    return this.readSplitVarint64(module$contents$jspb$utils_joinZigzag64)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readZigzagVarint64String = function () {
	    return this.readSplitZigzagVarint64(module$contents$jspb$utils_joinSignedDecimalString)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUint8 = function () {
	    var a = this.bytes_[this.cursor_ + 0];
	    this.cursor_ += 1;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return a
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUint16 = function () {
	    var a = this.bytes_[this.cursor_ + 0],
	        b = this.bytes_[this.cursor_ + 1];
	    this.cursor_ += 2;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return a << 0 | b << 8
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUint32 = function () {
	    var a = this.bytes_[this.cursor_ + 0],
	        b = this.bytes_[this.cursor_ + 1],
	        c = this.bytes_[this.cursor_ + 2],
	        d = this.bytes_[this.cursor_ + 3];
	    this.cursor_ += 4;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return (a << 0 | b << 8 | c << 16 | d << 24) >>> 0
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUint64 = function () {
	    var a = this.readUint32(),
	        b = this.readUint32();
	    return module$contents$jspb$utils_joinUint64(a, b)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readUint64String = function () {
	    var a = this.readUint32(),
	        b = this.readUint32();
	    return module$contents$jspb$utils_joinUnsignedDecimalString(a, b)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readInt8 = function () {
	    var a = this.bytes_[this.cursor_ + 0];
	    this.cursor_ += 1;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return a << 24 >> 24
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readInt16 = function () {
	    var a = this.bytes_[this.cursor_ + 0],
	        b = this.bytes_[this.cursor_ + 1];
	    this.cursor_ += 2;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return (a << 0 | b << 8) << 16 >> 16
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readInt32 = function () {
	    var a = this.bytes_[this.cursor_ + 0],
	        b = this.bytes_[this.cursor_ + 1],
	        c = this.bytes_[this.cursor_ + 2],
	        d = this.bytes_[this.cursor_ + 3];
	    this.cursor_ += 4;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return a << 0 | b << 8 | c << 16 | d << 24
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readInt64 = function () {
	    var a = this.readUint32(),
	        b = this.readUint32();
	    return module$contents$jspb$utils_joinInt64(a, b)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readInt64String = function () {
	    var a = this.readUint32(),
	        b = this.readUint32();
	    return module$contents$jspb$utils_joinSignedDecimalString(a, b)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readFloat = function () {
	    var a = this.readUint32();
	    return module$contents$jspb$utils_joinFloat32(a)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readDouble = function () {
	    var a = this.readUint32(),
	        b = this.readUint32();
	    return module$contents$jspb$utils_joinFloat64(a, b)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readBool = function () {
	    return !!this.bytes_[this.cursor_++]
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readEnum = function () {
	    return this.readSignedVarint32()
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readString = function (a) {
	    var b = this.bytes_,
	        c = this.cursor_;
	    a = c + a;
	    for (var d = [], e = ""; c < a;) {
	        var f = b[c++];
	        if (128 > f) d.push(f);
	        else if (192 > f) continue;
	        else if (224 > f) {
	            var g = b[c++];
	            d.push((f & 31) << 6 | g & 63);
	        } else if (240 > f) {
	            g = b[c++];
	            var h = b[c++];
	            d.push((f & 15) << 12 | (g & 63) << 6 | h & 63);
	        } else if (248 > f) {
	            g = b[c++];
	            h = b[c++];
	            var k = b[c++];
	            f = (f & 7) << 18 | (g & 63) << 12 | (h & 63) << 6 | k & 63;
	            f -= 65536;
	            d.push((f >> 10 & 1023) + 55296, (f & 1023) + 56320);
	        }
	        8192 <= d.length && (e += String.fromCharCode.apply(null,
	            d), d.length = 0);
	    }
	    e += goog.crypt.byteArrayToString(d);
	    this.cursor_ = c;
	    return e
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readStringWithLength = function () {
	    var a = this.readUnsignedVarint32();
	    return this.readString(a)
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.prototype.readBytes = function (a) {
	    if (0 > a || this.cursor_ + a > this.bytes_.length) return this.error_ = !0, goog.asserts.fail("Invalid byte length!"), new Uint8Array(0);
	    var b = this.bytes_.subarray(this.cursor_, this.cursor_ + a);
	    this.cursor_ += a;
	    goog.asserts.assert(this.cursor_ <= this.end_);
	    return b
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.resetInstanceCache = function () {
	    module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_ = [];
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.getInstanceCache = function () {
	    return module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_
	};
	module$contents$jspb$BinaryDecoder_BinaryDecoder.instanceCache_ = [];
	jspb.BinaryDecoder = module$contents$jspb$BinaryDecoder_BinaryDecoder;
	var module$contents$jspb$BinaryEncoder_BinaryEncoder = function () {
	    this.buffer_ = [];
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.length = function () {
	    return this.buffer_.length
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.end = function () {
	    var a = this.buffer_;
	    this.buffer_ = [];
	    return a
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeSplitVarint64 = function (a, b) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(b == Math.floor(b));
	    goog.asserts.assert(0 <= a && 4294967296 > a);
	    for (goog.asserts.assert(0 <= b && 4294967296 > b); 0 < b || 127 < a;) this.buffer_.push(a & 127 | 128), a = (a >>> 7 | b << 25) >>> 0, b >>>= 7;
	    this.buffer_.push(a);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeSplitFixed64 = function (a, b) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(b == Math.floor(b));
	    goog.asserts.assert(0 <= a && 4294967296 > a);
	    goog.asserts.assert(0 <= b && 4294967296 > b);
	    this.writeUint32(a);
	    this.writeUint32(b);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeSplitZigzagVarint64 = function (a, b) {
	    var c = this;
	    module$contents$jspb$utils_toZigzag64(a, b, function (d, e) {
	        c.writeSplitVarint64(d >>> 0, e >>> 0);
	    });
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeUnsignedVarint32 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    for (goog.asserts.assert(0 <= a && 4294967296 > a); 127 < a;) this.buffer_.push(a & 127 | 128), a >>>= 7;
	    this.buffer_.push(a);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeSignedVarint32 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-2147483648 <= a && 2147483648 > a);
	    if (0 <= a) this.writeUnsignedVarint32(a);
	    else {
	        for (var b = 0; 9 > b; b++) this.buffer_.push(a & 127 | 128), a >>= 7;
	        this.buffer_.push(1);
	    }
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeUnsignedVarint64 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(0 <= a && 1.8446744073709552E19 > a);
	    module$contents$jspb$utils_splitInt64(a);
	    this.writeSplitVarint64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeSignedVarint64 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-9223372036854775808 <= a && 0x7fffffffffffffff > a);
	    module$contents$jspb$utils_splitInt64(a);
	    this.writeSplitVarint64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeZigzagVarint32 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-2147483648 <= a && 2147483648 > a);
	    this.writeUnsignedVarint32((a << 1 ^ a >> 31) >>> 0);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeZigzagVarint64 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-9223372036854775808 <= a && 0x7fffffffffffffff > a);
	    module$contents$jspb$utils_splitZigzag64(a);
	    this.writeSplitVarint64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeZigzagVarint64String = function (a) {
	    var b = this;
	    module$contents$jspb$utils_splitDecimalString(a);
	    module$contents$jspb$utils_toZigzag64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High, function (c, d) {
	        b.writeSplitVarint64(c >>> 0, d >>> 0);
	    });
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeUint8 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(0 <= a && 256 > a);
	    this.buffer_.push(a >>> 0 & 255);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeUint16 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(0 <= a && 65536 > a);
	    this.buffer_.push(a >>> 0 & 255);
	    this.buffer_.push(a >>> 8 & 255);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeUint32 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(0 <= a && 4294967296 > a);
	    this.buffer_.push(a >>> 0 & 255);
	    this.buffer_.push(a >>> 8 & 255);
	    this.buffer_.push(a >>> 16 & 255);
	    this.buffer_.push(a >>> 24 & 255);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeUint64 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(0 <= a && 1.8446744073709552E19 > a);
	    module$contents$jspb$utils_splitUint64(a);
	    this.writeUint32(module$contents$jspb$utils_split64Low);
	    this.writeUint32(module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeInt8 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-128 <= a && 128 > a);
	    this.buffer_.push(a >>> 0 & 255);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeInt16 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-32768 <= a && 32768 > a);
	    this.buffer_.push(a >>> 0 & 255);
	    this.buffer_.push(a >>> 8 & 255);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeInt32 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-2147483648 <= a && 2147483648 > a);
	    this.buffer_.push(a >>> 0 & 255);
	    this.buffer_.push(a >>> 8 & 255);
	    this.buffer_.push(a >>> 16 & 255);
	    this.buffer_.push(a >>> 24 & 255);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeInt64 = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-9223372036854775808 <= a && 0x7fffffffffffffff > a);
	    module$contents$jspb$utils_splitInt64(a);
	    this.writeSplitFixed64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeInt64String = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-9223372036854775808 <= +a && 0x7fffffffffffffff > +a);
	    module$contents$jspb$utils_splitDecimalString(a);
	    this.writeSplitFixed64(module$contents$jspb$utils_split64Low, module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeFloat = function (a) {
	    goog.asserts.assert(Infinity === a || -Infinity === a || isNaN(a) || -3.4028234663852886E38 <= a && 3.4028234663852886E38 >= a);
	    module$contents$jspb$utils_splitFloat32(a);
	    this.writeUint32(module$contents$jspb$utils_split64Low);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeDouble = function (a) {
	    goog.asserts.assert(Infinity === a || -Infinity === a || isNaN(a) || -1.7976931348623157E308 <= a && 1.7976931348623157E308 >= a);
	    module$contents$jspb$utils_splitFloat64(a);
	    this.writeUint32(module$contents$jspb$utils_split64Low);
	    this.writeUint32(module$contents$jspb$utils_split64High);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeBool = function (a) {
	    goog.asserts.assert("boolean" === typeof a || "number" === typeof a);
	    this.buffer_.push(a ? 1 : 0);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeEnum = function (a) {
	    goog.asserts.assert(a == Math.floor(a));
	    goog.asserts.assert(-2147483648 <= a && 2147483648 > a);
	    this.writeSignedVarint32(a);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeBytes = function (a) {
	    this.buffer_.push.apply(this.buffer_, a);
	};
	module$contents$jspb$BinaryEncoder_BinaryEncoder.prototype.writeString = function (a) {
	    for (var b = this.buffer_.length, c = 0; c < a.length; c++) {
	        var d = a.charCodeAt(c);
	        if (128 > d) this.buffer_.push(d);
	        else if (2048 > d) this.buffer_.push(d >> 6 | 192), this.buffer_.push(d & 63 | 128);
	        else if (65536 > d)
	            if (55296 <= d && 56319 >= d && c + 1 < a.length) {
	                var e = a.charCodeAt(c + 1);
	                56320 <= e && 57343 >= e && (d = 1024 * (d - 55296) + e - 56320 + 65536, this.buffer_.push(d >> 18 | 240), this.buffer_.push(d >> 12 & 63 | 128), this.buffer_.push(d >> 6 & 63 | 128), this.buffer_.push(d & 63 | 128),
	                    c++);
	            } else this.buffer_.push(d >> 12 | 224), this.buffer_.push(d >> 6 & 63 | 128), this.buffer_.push(d & 63 | 128);
	    }
	    return this.buffer_.length - b
	};
	jspb.BinaryEncoder = module$contents$jspb$BinaryEncoder_BinaryEncoder;
	var module$contents$jspb$BinaryReader_BinaryReader = function (a, b, c) {
	    this.decoder_ = module$contents$jspb$BinaryDecoder_BinaryDecoder.alloc(a, b, c);
	    this.fieldCursor_ = this.decoder_.getCursor();
	    this.nextField_ = -1;
	    this.nextWireType_ = module$contents$jspb$BinaryConstants_WireType.INVALID;
	    this.error_ = !1;
	    this.readCallbacks_ = null;
	};
	module$contents$jspb$BinaryReader_BinaryReader.alloc = function (a, b, c) {
	    if (module$contents$jspb$BinaryReader_BinaryReader.instanceCache_.length) {
	        var d = module$contents$jspb$BinaryReader_BinaryReader.instanceCache_.pop();
	        a && d.decoder_.setBlock(a, b, c);
	        return d
	    }
	    return new module$contents$jspb$BinaryReader_BinaryReader(a, b, c)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.free = function () {
	    this.decoder_.clear();
	    this.nextField_ = -1;
	    this.nextWireType_ = module$contents$jspb$BinaryConstants_WireType.INVALID;
	    this.error_ = !1;
	    this.readCallbacks_ = null;
	    100 > module$contents$jspb$BinaryReader_BinaryReader.instanceCache_.length && module$contents$jspb$BinaryReader_BinaryReader.instanceCache_.push(this);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getFieldCursor = function () {
	    return this.fieldCursor_
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getCursor = function () {
	    return this.decoder_.getCursor()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getBuffer = function () {
	    return this.decoder_.getBuffer()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getFieldNumber = function () {
	    return this.nextField_
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getWireType = function () {
	    return this.nextWireType_
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.isEndGroup = function () {
	    return this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.END_GROUP
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.isDelimited = function () {
	    return this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.DELIMITED
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getError = function () {
	    return this.error_ || this.decoder_.getError()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.setBlock = function (a, b, c) {
	    this.decoder_.setBlock(a, b, c);
	    this.nextField_ = -1;
	    this.nextWireType_ = module$contents$jspb$BinaryConstants_WireType.INVALID;
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.reset = function () {
	    this.decoder_.reset();
	    this.nextField_ = -1;
	    this.nextWireType_ = module$contents$jspb$BinaryConstants_WireType.INVALID;
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.advance = function (a) {
	    this.decoder_.advance(a);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.nextField = function () {
	    if (this.decoder_.atEnd()) return !1;
	    if (this.getError()) return goog.asserts.fail("Decoder hit an error"), !1;
	    this.fieldCursor_ = this.decoder_.getCursor();
	    var a = this.decoder_.readUnsignedVarint32(),
	        b = a >>> 3;
	    a &= 7;
	    if (a != module$contents$jspb$BinaryConstants_WireType.VARINT && a != module$contents$jspb$BinaryConstants_WireType.FIXED32 && a != module$contents$jspb$BinaryConstants_WireType.FIXED64 && a != module$contents$jspb$BinaryConstants_WireType.DELIMITED &&
	        a != module$contents$jspb$BinaryConstants_WireType.START_GROUP && a != module$contents$jspb$BinaryConstants_WireType.END_GROUP) return goog.asserts.fail("Invalid wire type: %s (at position %s)", a, this.fieldCursor_), this.error_ = !0, !1;
	    this.nextField_ = b;
	    this.nextWireType_ = a;
	    return !0
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.unskipHeader = function () {
	    this.decoder_.unskipVarint(this.nextField_ << 3 | this.nextWireType_);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipMatchingFields = function () {
	    var a = this.nextField_;
	    for (this.unskipHeader(); this.nextField() && this.getFieldNumber() == a;) this.skipField();
	    this.decoder_.atEnd() || this.unskipHeader();
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipVarintField = function () {
	    this.nextWireType_ != module$contents$jspb$BinaryConstants_WireType.VARINT ? (goog.asserts.fail("Invalid wire type for skipVarintField"), this.skipField()) : this.decoder_.skipVarint();
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipDelimitedField = function () {
	    if (this.nextWireType_ != module$contents$jspb$BinaryConstants_WireType.DELIMITED) goog.asserts.fail("Invalid wire type for skipDelimitedField"), this.skipField();
	    else {
	        var a = this.decoder_.readUnsignedVarint32();
	        this.decoder_.advance(a);
	    }
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipFixed32Field = function () {
	    this.nextWireType_ != module$contents$jspb$BinaryConstants_WireType.FIXED32 ? (goog.asserts.fail("Invalid wire type for skipFixed32Field"), this.skipField()) : this.decoder_.advance(4);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipFixed64Field = function () {
	    this.nextWireType_ != module$contents$jspb$BinaryConstants_WireType.FIXED64 ? (goog.asserts.fail("Invalid wire type for skipFixed64Field"), this.skipField()) : this.decoder_.advance(8);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipGroup = function () {
	    var a = this.nextField_;
	    do {
	        if (!this.nextField()) {
	            goog.asserts.fail("Unmatched start-group tag: stream EOF");
	            this.error_ = !0;
	            break
	        }
	        if (this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.END_GROUP) {
	            this.nextField_ != a && (goog.asserts.fail("Unmatched end-group tag"), this.error_ = !0);
	            break
	        }
	        this.skipField();
	    } while (1)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.skipField = function () {
	    switch (this.nextWireType_) {
	        case module$contents$jspb$BinaryConstants_WireType.VARINT:
	            this.skipVarintField();
	            break;
	        case module$contents$jspb$BinaryConstants_WireType.FIXED64:
	            this.skipFixed64Field();
	            break;
	        case module$contents$jspb$BinaryConstants_WireType.DELIMITED:
	            this.skipDelimitedField();
	            break;
	        case module$contents$jspb$BinaryConstants_WireType.FIXED32:
	            this.skipFixed32Field();
	            break;
	        case module$contents$jspb$BinaryConstants_WireType.START_GROUP:
	            this.skipGroup();
	            break;
	        default:
	            this.error_ = !0, goog.asserts.fail("Invalid wire encoding for field.");
	    }
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.registerReadCallback = function (a, b) {
	    null === this.readCallbacks_ && (this.readCallbacks_ = {});
	    goog.asserts.assert(!this.readCallbacks_[a]);
	    this.readCallbacks_[a] = b;
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.runReadCallback = function (a) {
	    goog.asserts.assert(null !== this.readCallbacks_);
	    a = this.readCallbacks_[a];
	    goog.asserts.assert(a);
	    return a(this)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readAny = function (a) {
	    this.nextWireType_ = module$contents$jspb$BinaryConstants_FieldTypeToWireType(a);
	    switch (a) {
	        case module$contents$jspb$BinaryConstants_FieldType.DOUBLE:
	            return this.readDouble();
	        case module$contents$jspb$BinaryConstants_FieldType.FLOAT:
	            return this.readFloat();
	        case module$contents$jspb$BinaryConstants_FieldType.INT64:
	            return this.readInt64();
	        case module$contents$jspb$BinaryConstants_FieldType.UINT64:
	            return this.readUint64();
	        case module$contents$jspb$BinaryConstants_FieldType.INT32:
	            return this.readInt32();
	        case module$contents$jspb$BinaryConstants_FieldType.FIXED64:
	            return this.readFixed64();
	        case module$contents$jspb$BinaryConstants_FieldType.FIXED32:
	            return this.readFixed32();
	        case module$contents$jspb$BinaryConstants_FieldType.BOOL:
	            return this.readBool();
	        case module$contents$jspb$BinaryConstants_FieldType.STRING:
	            return this.readString();
	        case module$contents$jspb$BinaryConstants_FieldType.GROUP:
	            goog.asserts.fail("Group field type not supported in readAny()");
	        case module$contents$jspb$BinaryConstants_FieldType.MESSAGE:
	            goog.asserts.fail("Message field type not supported in readAny()");
	        case module$contents$jspb$BinaryConstants_FieldType.BYTES:
	            return this.readBytes();
	        case module$contents$jspb$BinaryConstants_FieldType.UINT32:
	            return this.readUint32();
	        case module$contents$jspb$BinaryConstants_FieldType.ENUM:
	            return this.readEnum();
	        case module$contents$jspb$BinaryConstants_FieldType.SFIXED32:
	            return this.readSfixed32();
	        case module$contents$jspb$BinaryConstants_FieldType.SFIXED64:
	            return this.readSfixed64();
	        case module$contents$jspb$BinaryConstants_FieldType.SINT32:
	            return this.readSint32();
	        case module$contents$jspb$BinaryConstants_FieldType.SINT64:
	            return this.readSint64();
	        default:
	            goog.asserts.fail("Invalid field type in readAny()");
	    }
	    return 0
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readMessage = function (a, b) {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.DELIMITED);
	    var c = this.decoder_.getEnd(),
	        d = this.decoder_.readUnsignedVarint32();
	    d = this.decoder_.getCursor() + d;
	    this.decoder_.setEnd(d);
	    b(a, this);
	    this.decoder_.setCursor(d);
	    this.decoder_.setEnd(c);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readGroup = function (a, b, c) {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.START_GROUP);
	    goog.asserts.assert(this.nextField_ == a);
	    c(b, this);
	    this.error_ || this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.END_GROUP || (goog.asserts.fail("Group submessage did not end with an END_GROUP tag"), this.error_ = !0);
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.getFieldDecoder = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.DELIMITED);
	    var a = this.decoder_.readUnsignedVarint32(),
	        b = this.decoder_.getCursor(),
	        c = b + a;
	    a = module$contents$jspb$BinaryDecoder_BinaryDecoder.alloc(this.decoder_.getBuffer(), b, a);
	    this.decoder_.setCursor(c);
	    return a
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readInt32 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSignedVarint32()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readInt32String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSignedVarint32String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readInt64 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSignedVarint64()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readInt64String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSignedVarint64String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readUint32 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readUnsignedVarint32()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readUint32String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readUnsignedVarint32String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readUint64 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readUnsignedVarint64()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readUint64String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readUnsignedVarint64String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSint32 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readZigzagVarint32()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSint64 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readZigzagVarint64()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSint64String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readZigzagVarint64String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readFixed32 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED32);
	    return this.decoder_.readUint32()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readFixed64 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    return this.decoder_.readUint64()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readFixed64String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    return this.decoder_.readUint64String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSfixed32 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED32);
	    return this.decoder_.readInt32()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSfixed32String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED32);
	    return this.decoder_.readInt32().toString()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSfixed64 = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    return this.decoder_.readInt64()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSfixed64String = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    return this.decoder_.readInt64String()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readFloat = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED32);
	    return this.decoder_.readFloat()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readDouble = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    return this.decoder_.readDouble()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readBool = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return !!this.decoder_.readUnsignedVarint32()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readEnum = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSignedVarint64()
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readString = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.DELIMITED);
	    var a = this.decoder_.readUnsignedVarint32();
	    return this.decoder_.readString(a)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readBytes = function () {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.DELIMITED);
	    var a = this.decoder_.readUnsignedVarint32();
	    return this.decoder_.readBytes(a)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSplitVarint64 = function (a) {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSplitVarint64(a)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSplitZigzagVarint64 = function (a) {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.VARINT);
	    return this.decoder_.readSplitVarint64(function (b, c) {
	        return module$contents$jspb$utils_fromZigzag64(b, c, a)
	    })
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readSplitFixed64 = function (a) {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    return this.decoder_.readSplitFixed64(a)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedField_ = function (a) {
	    goog.asserts.assert(this.nextWireType_ == module$contents$jspb$BinaryConstants_WireType.DELIMITED);
	    var b = this.decoder_.readUnsignedVarint32();
	    b = this.decoder_.getCursor() + b;
	    for (var c = []; this.decoder_.getCursor() < b;) c.push(a.call(this.decoder_));
	    return c
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedInt32 = function () {
	    return this.readPackedField_(this.decoder_.readSignedVarint32)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedInt32String = function () {
	    return this.readPackedField_(this.decoder_.readSignedVarint32String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedInt64 = function () {
	    return this.readPackedField_(this.decoder_.readSignedVarint64)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedInt64String = function () {
	    return this.readPackedField_(this.decoder_.readSignedVarint64String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedUint32 = function () {
	    return this.readPackedField_(this.decoder_.readUnsignedVarint32)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedUint32String = function () {
	    return this.readPackedField_(this.decoder_.readUnsignedVarint32String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedUint64 = function () {
	    return this.readPackedField_(this.decoder_.readUnsignedVarint64)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedUint64String = function () {
	    return this.readPackedField_(this.decoder_.readUnsignedVarint64String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedSint32 = function () {
	    return this.readPackedField_(this.decoder_.readZigzagVarint32)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedSint64 = function () {
	    return this.readPackedField_(this.decoder_.readZigzagVarint64)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedSint64String = function () {
	    return this.readPackedField_(this.decoder_.readZigzagVarint64String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedFixed32 = function () {
	    return this.readPackedField_(this.decoder_.readUint32)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedFixed64 = function () {
	    return this.readPackedField_(this.decoder_.readUint64)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedFixed64String = function () {
	    return this.readPackedField_(this.decoder_.readUint64String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedSfixed32 = function () {
	    return this.readPackedField_(this.decoder_.readInt32)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedSfixed64 = function () {
	    return this.readPackedField_(this.decoder_.readInt64)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedSfixed64String = function () {
	    return this.readPackedField_(this.decoder_.readInt64String)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedFloat = function () {
	    return this.readPackedField_(this.decoder_.readFloat)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedDouble = function () {
	    return this.readPackedField_(this.decoder_.readDouble)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedBool = function () {
	    return this.readPackedField_(this.decoder_.readBool)
	};
	module$contents$jspb$BinaryReader_BinaryReader.prototype.readPackedEnum = function () {
	    return this.readPackedField_(this.decoder_.readEnum)
	};
	module$contents$jspb$BinaryReader_BinaryReader.resetInstanceCache = function () {
	    module$contents$jspb$BinaryReader_BinaryReader.instanceCache_ = [];
	};
	module$contents$jspb$BinaryReader_BinaryReader.getInstanceCache = function () {
	    return module$contents$jspb$BinaryReader_BinaryReader.instanceCache_
	};
	module$contents$jspb$BinaryReader_BinaryReader.instanceCache_ = [];
	jspb.BinaryReader = module$contents$jspb$BinaryReader_BinaryReader;
	goog.labs.userAgent.platform = {};
	goog.labs.userAgent.platform.isAndroid = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Android")
	};
	goog.labs.userAgent.platform.isIpod = function () {
	    return goog.labs.userAgent.util.matchUserAgent("iPod")
	};
	goog.labs.userAgent.platform.isIphone = function () {
	    return goog.labs.userAgent.util.matchUserAgent("iPhone") && !goog.labs.userAgent.util.matchUserAgent("iPod") && !goog.labs.userAgent.util.matchUserAgent("iPad")
	};
	goog.labs.userAgent.platform.isIpad = function () {
	    return goog.labs.userAgent.util.matchUserAgent("iPad")
	};
	goog.labs.userAgent.platform.isIos = function () {
	    return goog.labs.userAgent.platform.isIphone() || goog.labs.userAgent.platform.isIpad() || goog.labs.userAgent.platform.isIpod()
	};
	goog.labs.userAgent.platform.isMacintosh = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Macintosh")
	};
	goog.labs.userAgent.platform.isLinux = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Linux")
	};
	goog.labs.userAgent.platform.isWindows = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Windows")
	};
	goog.labs.userAgent.platform.isChromeOS = function () {
	    return goog.labs.userAgent.util.matchUserAgent("CrOS")
	};
	goog.labs.userAgent.platform.isChromecast = function () {
	    return goog.labs.userAgent.util.matchUserAgent("CrKey")
	};
	goog.labs.userAgent.platform.isKaiOS = function () {
	    return goog.labs.userAgent.util.matchUserAgentIgnoreCase("KaiOS")
	};
	goog.labs.userAgent.platform.getVersion = function () {
	    var a = goog.labs.userAgent.util.getUserAgent(),
	        b = "";
	    goog.labs.userAgent.platform.isWindows() ? (b = /Windows (?:NT|Phone) ([0-9.]+)/, b = (a = b.exec(a)) ? a[1] : "0.0") : goog.labs.userAgent.platform.isIos() ? (b = /(?:iPhone|iPod|iPad|CPU)\s+OS\s+(\S+)/, b = (a = b.exec(a)) && a[1].replace(/_/g, ".")) : goog.labs.userAgent.platform.isMacintosh() ? (b = /Mac OS X ([0-9_.]+)/, b = (a = b.exec(a)) ? a[1].replace(/_/g, ".") : "10") : goog.labs.userAgent.platform.isKaiOS() ? (b = /(?:KaiOS)\/(\S+)/i,
	        b = (a = b.exec(a)) && a[1]) : goog.labs.userAgent.platform.isAndroid() ? (b = /Android\s+([^\);]+)(\)|;)/, b = (a = b.exec(a)) && a[1]) : goog.labs.userAgent.platform.isChromeOS() && (b = /(?:CrOS\s+(?:i686|x86_64)\s+([0-9.]+))/, b = (a = b.exec(a)) && a[1]);
	    return b || ""
	};
	goog.labs.userAgent.platform.isVersionOrHigher = function (a) {
	    return 0 <= goog.string.compareVersions(goog.labs.userAgent.platform.getVersion(), a)
	};
	goog.labs.userAgent.engine = {};
	goog.labs.userAgent.engine.isPresto = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Presto")
	};
	goog.labs.userAgent.engine.isTrident = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Trident") || goog.labs.userAgent.util.matchUserAgent("MSIE")
	};
	goog.labs.userAgent.engine.isEdge = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Edge")
	};
	goog.labs.userAgent.engine.isWebKit = function () {
	    return goog.labs.userAgent.util.matchUserAgentIgnoreCase("WebKit") && !goog.labs.userAgent.engine.isEdge()
	};
	goog.labs.userAgent.engine.isGecko = function () {
	    return goog.labs.userAgent.util.matchUserAgent("Gecko") && !goog.labs.userAgent.engine.isWebKit() && !goog.labs.userAgent.engine.isTrident() && !goog.labs.userAgent.engine.isEdge()
	};
	goog.labs.userAgent.engine.getVersion = function () {
	    var a = goog.labs.userAgent.util.getUserAgent();
	    if (a) {
	        a = goog.labs.userAgent.util.extractVersionTuples(a);
	        var b = goog.labs.userAgent.engine.getEngineTuple_(a);
	        if (b) return "Gecko" == b[0] ? goog.labs.userAgent.engine.getVersionForKey_(a, "Firefox") : b[1];
	        a = a[0];
	        var c;
	        if (a && (c = a[2]) && (c = /Trident\/([^\s;]+)/.exec(c))) return c[1]
	    }
	    return ""
	};
	goog.labs.userAgent.engine.getEngineTuple_ = function (a) {
	    if (!goog.labs.userAgent.engine.isEdge()) return a[1];
	    for (var b = 0; b < a.length; b++) {
	        var c = a[b];
	        if ("Edge" == c[0]) return c
	    }
	};
	goog.labs.userAgent.engine.isVersionOrHigher = function (a) {
	    return 0 <= goog.string.compareVersions(goog.labs.userAgent.engine.getVersion(), a)
	};
	goog.labs.userAgent.engine.getVersionForKey_ = function (a, b) {
	    return (a = module$contents$goog$array_find(a, function (c) {
	        return b == c[0]
	    })) && a[1] || ""
	};
	goog.reflect = {};
	goog.reflect.object = function (a, b) {
	    return b
	};
	goog.reflect.objectProperty = function (a) {
	    return a
	};
	goog.reflect.sinkValue = function (a) {
	    goog.reflect.sinkValue[" "](a);
	    return a
	};
	goog.reflect.sinkValue[" "] = goog.nullFunction;
	goog.reflect.canAccessProperty = function (a, b) {
	    try {
	        return goog.reflect.sinkValue(a[b]), !0
	    } catch (c) {}
	    return !1
	};
	goog.reflect.cache = function (a, b, c, d) {
	    d = d ? d(b) : b;
	    return Object.prototype.hasOwnProperty.call(a, d) ? a[d] : a[d] = c(b)
	};
	goog.userAgent = {};
	goog.userAgent.ASSUME_IE = !1;
	goog.userAgent.ASSUME_EDGE = !1;
	goog.userAgent.ASSUME_GECKO = !1;
	goog.userAgent.ASSUME_WEBKIT = !1;
	goog.userAgent.ASSUME_MOBILE_WEBKIT = !1;
	goog.userAgent.ASSUME_OPERA = !1;
	goog.userAgent.ASSUME_ANY_VERSION = !1;
	goog.userAgent.BROWSER_KNOWN_ = goog.userAgent.ASSUME_IE || goog.userAgent.ASSUME_EDGE || goog.userAgent.ASSUME_GECKO || goog.userAgent.ASSUME_MOBILE_WEBKIT || goog.userAgent.ASSUME_WEBKIT || goog.userAgent.ASSUME_OPERA;
	goog.userAgent.getUserAgentString = function () {
	    return goog.labs.userAgent.util.getUserAgent()
	};
	goog.userAgent.getNavigatorTyped = function () {
	    return goog.global.navigator || null
	};
	goog.userAgent.getNavigator = function () {
	    return goog.userAgent.getNavigatorTyped()
	};
	goog.userAgent.OPERA = goog.userAgent.BROWSER_KNOWN_ ? goog.userAgent.ASSUME_OPERA : goog.labs.userAgent.browser.isOpera();
	goog.userAgent.IE = goog.userAgent.BROWSER_KNOWN_ ? goog.userAgent.ASSUME_IE : goog.labs.userAgent.browser.isIE();
	goog.userAgent.EDGE = goog.userAgent.BROWSER_KNOWN_ ? goog.userAgent.ASSUME_EDGE : goog.labs.userAgent.engine.isEdge();
	goog.userAgent.EDGE_OR_IE = goog.userAgent.EDGE || goog.userAgent.IE;
	goog.userAgent.GECKO = goog.userAgent.BROWSER_KNOWN_ ? goog.userAgent.ASSUME_GECKO : goog.labs.userAgent.engine.isGecko();
	goog.userAgent.WEBKIT = goog.userAgent.BROWSER_KNOWN_ ? goog.userAgent.ASSUME_WEBKIT || goog.userAgent.ASSUME_MOBILE_WEBKIT : goog.labs.userAgent.engine.isWebKit();
	goog.userAgent.isMobile_ = function () {
	    return goog.userAgent.WEBKIT && goog.labs.userAgent.util.matchUserAgent("Mobile")
	};
	goog.userAgent.MOBILE = goog.userAgent.ASSUME_MOBILE_WEBKIT || goog.userAgent.isMobile_();
	goog.userAgent.SAFARI = goog.userAgent.WEBKIT;
	goog.userAgent.determinePlatform_ = function () {
	    var a = goog.userAgent.getNavigatorTyped();
	    return a && a.platform || ""
	};
	goog.userAgent.PLATFORM = goog.userAgent.determinePlatform_();
	goog.userAgent.ASSUME_MAC = !1;
	goog.userAgent.ASSUME_WINDOWS = !1;
	goog.userAgent.ASSUME_LINUX = !1;
	goog.userAgent.ASSUME_X11 = !1;
	goog.userAgent.ASSUME_ANDROID = !1;
	goog.userAgent.ASSUME_IPHONE = !1;
	goog.userAgent.ASSUME_IPAD = !1;
	goog.userAgent.ASSUME_IPOD = !1;
	goog.userAgent.ASSUME_KAIOS = !1;
	goog.userAgent.PLATFORM_KNOWN_ = goog.userAgent.ASSUME_MAC || goog.userAgent.ASSUME_WINDOWS || goog.userAgent.ASSUME_LINUX || goog.userAgent.ASSUME_X11 || goog.userAgent.ASSUME_ANDROID || goog.userAgent.ASSUME_IPHONE || goog.userAgent.ASSUME_IPAD || goog.userAgent.ASSUME_IPOD;
	goog.userAgent.MAC = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_MAC : goog.labs.userAgent.platform.isMacintosh();
	goog.userAgent.WINDOWS = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_WINDOWS : goog.labs.userAgent.platform.isWindows();
	goog.userAgent.isLegacyLinux_ = function () {
	    return goog.labs.userAgent.platform.isLinux() || goog.labs.userAgent.platform.isChromeOS()
	};
	goog.userAgent.LINUX = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_LINUX : goog.userAgent.isLegacyLinux_();
	goog.userAgent.isX11_ = function () {
	    var a = goog.userAgent.getNavigatorTyped();
	    return !!a && goog.string.contains(a.appVersion || "", "X11")
	};
	goog.userAgent.X11 = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_X11 : goog.userAgent.isX11_();
	goog.userAgent.ANDROID = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_ANDROID : goog.labs.userAgent.platform.isAndroid();
	goog.userAgent.IPHONE = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_IPHONE : goog.labs.userAgent.platform.isIphone();
	goog.userAgent.IPAD = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_IPAD : goog.labs.userAgent.platform.isIpad();
	goog.userAgent.IPOD = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_IPOD : goog.labs.userAgent.platform.isIpod();
	goog.userAgent.IOS = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_IPHONE || goog.userAgent.ASSUME_IPAD || goog.userAgent.ASSUME_IPOD : goog.labs.userAgent.platform.isIos();
	goog.userAgent.KAIOS = goog.userAgent.PLATFORM_KNOWN_ ? goog.userAgent.ASSUME_KAIOS : goog.labs.userAgent.platform.isKaiOS();
	goog.userAgent.determineVersion_ = function () {
	    var a = "",
	        b = goog.userAgent.getVersionRegexResult_();
	    b && (a = b ? b[1] : "");
	    return goog.userAgent.IE && (b = goog.userAgent.getDocumentMode_(), null != b && b > parseFloat(a)) ? String(b) : a
	};
	goog.userAgent.getVersionRegexResult_ = function () {
	    var a = goog.userAgent.getUserAgentString();
	    if (goog.userAgent.GECKO) return /rv:([^\);]+)(\)|;)/.exec(a);
	    if (goog.userAgent.EDGE) return /Edge\/([\d\.]+)/.exec(a);
	    if (goog.userAgent.IE) return /\b(?:MSIE|rv)[: ]([^\);]+)(\)|;)/.exec(a);
	    if (goog.userAgent.WEBKIT) return /WebKit\/(\S+)/.exec(a);
	    if (goog.userAgent.OPERA) return /(?:Version)[ \/]?(\S+)/.exec(a)
	};
	goog.userAgent.getDocumentMode_ = function () {
	    var a = goog.global.document;
	    return a ? a.documentMode : void 0
	};
	goog.userAgent.VERSION = goog.userAgent.determineVersion_();
	goog.userAgent.compare = function (a, b) {
	    return goog.string.compareVersions(a, b)
	};
	goog.userAgent.isVersionOrHigherCache_ = {};
	goog.userAgent.isVersionOrHigher = function (a) {
	    return goog.userAgent.ASSUME_ANY_VERSION || goog.reflect.cache(goog.userAgent.isVersionOrHigherCache_, a, function () {
	        return 0 <= goog.string.compareVersions(goog.userAgent.VERSION, a)
	    })
	};
	goog.userAgent.isVersion = goog.userAgent.isVersionOrHigher;
	goog.userAgent.isDocumentModeOrHigher = function (a) {
	    return Number(goog.userAgent.DOCUMENT_MODE) >= a
	};
	goog.userAgent.isDocumentMode = goog.userAgent.isDocumentModeOrHigher;
	var JSCompiler_inline_result$jscomp$44;
	if (goog.global.document && goog.userAgent.IE) {
	    var documentMode$jscomp$inline_52 = goog.userAgent.getDocumentMode_();
	    JSCompiler_inline_result$jscomp$44 = documentMode$jscomp$inline_52 ? documentMode$jscomp$inline_52 : parseInt(goog.userAgent.VERSION, 10) || void 0;
	} else JSCompiler_inline_result$jscomp$44 = void 0;
	goog.userAgent.DOCUMENT_MODE = JSCompiler_inline_result$jscomp$44;
	goog.userAgent.product = {};
	goog.userAgent.product.ASSUME_FIREFOX = !1;
	goog.userAgent.product.ASSUME_IPHONE = !1;
	goog.userAgent.product.ASSUME_IPAD = !1;
	goog.userAgent.product.ASSUME_ANDROID = !1;
	goog.userAgent.product.ASSUME_CHROME = !1;
	goog.userAgent.product.ASSUME_SAFARI = !1;
	goog.userAgent.product.PRODUCT_KNOWN_ = goog.userAgent.ASSUME_IE || goog.userAgent.ASSUME_EDGE || goog.userAgent.ASSUME_OPERA || goog.userAgent.product.ASSUME_FIREFOX || goog.userAgent.product.ASSUME_IPHONE || goog.userAgent.product.ASSUME_IPAD || goog.userAgent.product.ASSUME_ANDROID || goog.userAgent.product.ASSUME_CHROME || goog.userAgent.product.ASSUME_SAFARI;
	goog.userAgent.product.OPERA = goog.userAgent.OPERA;
	goog.userAgent.product.IE = goog.userAgent.IE;
	goog.userAgent.product.EDGE = goog.userAgent.EDGE;
	goog.userAgent.product.FIREFOX = goog.userAgent.product.PRODUCT_KNOWN_ ? goog.userAgent.product.ASSUME_FIREFOX : goog.labs.userAgent.browser.isFirefox();
	goog.userAgent.product.isIphoneOrIpod_ = function () {
	    return goog.labs.userAgent.platform.isIphone() || goog.labs.userAgent.platform.isIpod()
	};
	goog.userAgent.product.IPHONE = goog.userAgent.product.PRODUCT_KNOWN_ ? goog.userAgent.product.ASSUME_IPHONE : goog.userAgent.product.isIphoneOrIpod_();
	goog.userAgent.product.IPAD = goog.userAgent.product.PRODUCT_KNOWN_ ? goog.userAgent.product.ASSUME_IPAD : goog.labs.userAgent.platform.isIpad();
	goog.userAgent.product.ANDROID = goog.userAgent.product.PRODUCT_KNOWN_ ? goog.userAgent.product.ASSUME_ANDROID : goog.labs.userAgent.browser.isAndroidBrowser();
	goog.userAgent.product.CHROME = goog.userAgent.product.PRODUCT_KNOWN_ ? goog.userAgent.product.ASSUME_CHROME : goog.labs.userAgent.browser.isChrome();
	goog.userAgent.product.isSafariDesktop_ = function () {
	    return goog.labs.userAgent.browser.isSafari() && !goog.labs.userAgent.platform.isIos()
	};
	goog.userAgent.product.SAFARI = goog.userAgent.product.PRODUCT_KNOWN_ ? goog.userAgent.product.ASSUME_SAFARI : goog.userAgent.product.isSafariDesktop_();
	goog.crypt.base64 = {};
	goog.crypt.base64.DEFAULT_ALPHABET_COMMON_ = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	goog.crypt.base64.ENCODED_VALS = goog.crypt.base64.DEFAULT_ALPHABET_COMMON_ + "+/=";
	goog.crypt.base64.ENCODED_VALS_WEBSAFE = goog.crypt.base64.DEFAULT_ALPHABET_COMMON_ + "-_.";
	goog.crypt.base64.Alphabet = {
	    DEFAULT: 0,
	    NO_PADDING: 1,
	    WEBSAFE: 2,
	    WEBSAFE_DOT_PADDING: 3,
	    WEBSAFE_NO_PADDING: 4
	};
	goog.crypt.base64.paddingChars_ = "=.";
	goog.crypt.base64.isPadding_ = function (a) {
	    return goog.string.contains(goog.crypt.base64.paddingChars_, a)
	};
	goog.crypt.base64.byteToCharMaps_ = {};
	goog.crypt.base64.charToByteMap_ = null;
	goog.crypt.base64.ASSUME_NATIVE_SUPPORT_ = goog.userAgent.GECKO || goog.userAgent.WEBKIT && !goog.userAgent.product.SAFARI || goog.userAgent.OPERA;
	goog.crypt.base64.HAS_NATIVE_ENCODE_ = goog.crypt.base64.ASSUME_NATIVE_SUPPORT_ || "function" == typeof goog.global.btoa;
	goog.crypt.base64.HAS_NATIVE_DECODE_ = goog.crypt.base64.ASSUME_NATIVE_SUPPORT_ || !goog.userAgent.product.SAFARI && !goog.userAgent.IE && "function" == typeof goog.global.atob;
	goog.crypt.base64.encodeByteArray = function (a, b) {
	    goog.asserts.assert(goog.isArrayLike(a), "encodeByteArray takes an array as a parameter");
	    void 0 === b && (b = goog.crypt.base64.Alphabet.DEFAULT);
	    goog.crypt.base64.init_();
	    b = goog.crypt.base64.byteToCharMaps_[b];
	    for (var c = [], d = 0; d < a.length; d += 3) {
	        var e = a[d],
	            f = d + 1 < a.length,
	            g = f ? a[d + 1] : 0,
	            h = d + 2 < a.length,
	            k = h ? a[d + 2] : 0,
	            l = e >> 2;
	        e = (e & 3) << 4 | g >> 4;
	        g = (g & 15) << 2 | k >> 6;
	        k &= 63;
	        h || (k = 64, f || (g = 64));
	        c.push(b[l], b[e], b[g] || "", b[k] || "");
	    }
	    return c.join("")
	};
	goog.crypt.base64.encodeString = function (a, b) {
	    return goog.crypt.base64.HAS_NATIVE_ENCODE_ && !b ? goog.global.btoa(a) : goog.crypt.base64.encodeByteArray(goog.crypt.stringToByteArray(a), b)
	};
	goog.crypt.base64.decodeString = function (a, b) {
	    if (goog.crypt.base64.HAS_NATIVE_DECODE_ && !b) return goog.global.atob(a);
	    var c = "";
	    goog.crypt.base64.decodeStringInternal_(a, function (d) {
	        c += String.fromCharCode(d);
	    });
	    return c
	};
	goog.crypt.base64.decodeStringToByteArray = function (a) {
	    var b = [];
	    goog.crypt.base64.decodeStringInternal_(a, function (c) {
	        b.push(c);
	    });
	    return b
	};
	goog.crypt.base64.decodeStringToUint8Array = function (a) {
	    goog.asserts.assert(!goog.userAgent.IE || goog.userAgent.isVersionOrHigher("10"), "Browser does not support typed arrays");
	    var b = a.length,
	        c = 3 * b / 4;
	    c % 3 ? c = Math.floor(c) : goog.crypt.base64.isPadding_(a[b - 1]) && (c = goog.crypt.base64.isPadding_(a[b - 2]) ? c - 2 : c - 1);
	    var d = new Uint8Array(c),
	        e = 0;
	    goog.crypt.base64.decodeStringInternal_(a, function (f) {
	        d[e++] = f;
	    });
	    return d.subarray(0, e)
	};
	goog.crypt.base64.decodeStringInternal_ = function (a, b) {
	    function c(k) {
	        for (; d < a.length;) {
	            var l = a.charAt(d++),
	                m = goog.crypt.base64.charToByteMap_[l];
	            if (null != m) return m;
	            if (!goog.string.isEmptyOrWhitespace(l)) throw Error("Unknown base64 encoding at char: " + l);
	        }
	        return k
	    }
	    goog.crypt.base64.init_();
	    for (var d = 0;;) {
	        var e = c(-1),
	            f = c(0),
	            g = c(64),
	            h = c(64);
	        if (64 === h && -1 === e) break;
	        b(e << 2 | f >> 4);
	        64 != g && (b(f << 4 & 240 | g >> 2), 64 != h && b(g << 6 & 192 | h));
	    }
	};
	goog.crypt.base64.init_ = function () {
	    if (!goog.crypt.base64.charToByteMap_) {
	        goog.crypt.base64.charToByteMap_ = {};
	        for (var a = goog.crypt.base64.DEFAULT_ALPHABET_COMMON_.split(""), b = ["+/=", "+/", "-_=", "-_.", "-_"], c = 0; 5 > c; c++) {
	            var d = a.concat(b[c].split(""));
	            goog.crypt.base64.byteToCharMaps_[c] = d;
	            for (var e = 0; e < d.length; e++) {
	                var f = d[e],
	                    g = goog.crypt.base64.charToByteMap_[f];
	                void 0 === g ? goog.crypt.base64.charToByteMap_[f] = e : goog.asserts.assert(g === e);
	            }
	        }
	    }
	};
	var module$contents$jspb$BinaryWriter_BinaryWriter = function () {
	    this.blocks_ = [];
	    this.totalLength_ = 0;
	    this.encoder_ = new module$contents$jspb$BinaryEncoder_BinaryEncoder;
	    this.bookmarks_ = [];
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.appendUint8Array_ = function (a) {
	    var b = this.encoder_.end();
	    this.blocks_.push(b);
	    this.blocks_.push(a);
	    this.totalLength_ += b.length + a.length;
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.beginDelimited_ = function (a) {
	    this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED);
	    a = this.encoder_.end();
	    this.blocks_.push(a);
	    this.totalLength_ += a.length;
	    a.push(this.totalLength_);
	    return a
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.endDelimited_ = function (a) {
	    var b = a.pop();
	    b = this.totalLength_ + this.encoder_.length() - b;
	    for ((0, goog.asserts.assert)(0 <= b); 127 < b;) a.push(b & 127 | 128), b >>>= 7, this.totalLength_++;
	    a.push(b);
	    this.totalLength_++;
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSerializedMessage = function (a, b, c) {
	    this.appendUint8Array_(a.subarray(b, c));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.maybeWriteSerializedMessage = function (a, b, c) {
	    null != a && null != b && null != c && this.writeSerializedMessage(a, b, c);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.reset = function () {
	    this.blocks_ = [];
	    this.encoder_.end();
	    this.totalLength_ = 0;
	    this.bookmarks_ = [];
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.getResultBuffer = function () {
	    (0, goog.asserts.assert)(0 == this.bookmarks_.length);
	    for (var a = new Uint8Array(this.totalLength_ + this.encoder_.length()), b = this.blocks_, c = b.length, d = 0, e = 0; e < c; e++) {
	        var f = b[e];
	        a.set(f, d);
	        d += f.length;
	    }
	    b = this.encoder_.end();
	    a.set(b, d);
	    d += b.length;
	    (0, goog.asserts.assert)(d == a.length);
	    this.blocks_ = [a];
	    return a
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.getResultBase64String = function (a) {
	    return (0, goog.crypt.base64.encodeByteArray)(this.getResultBuffer(), a)
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.beginSubMessage = function (a) {
	    this.bookmarks_.push(this.beginDelimited_(a));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.endSubMessage = function () {
	    (0, goog.asserts.assert)(0 <= this.bookmarks_.length);
	    this.endDelimited_(this.bookmarks_.pop());
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeFieldHeader_ = function (a, b) {
	    (0, goog.asserts.assert)(1 <= a && a == Math.floor(a));
	    this.encoder_.writeUnsignedVarint32(8 * a + b);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeAny = function (a, b, c) {
	    switch (a) {
	        case module$contents$jspb$BinaryConstants_FieldType.DOUBLE:
	            this.writeDouble(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.FLOAT:
	            this.writeFloat(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.INT64:
	            this.writeInt64(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.UINT64:
	            this.writeUint64(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.INT32:
	            this.writeInt32(b,
	                c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.FIXED64:
	            this.writeFixed64(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.FIXED32:
	            this.writeFixed32(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.BOOL:
	            this.writeBool(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.STRING:
	            this.writeString(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.GROUP:
	            (0, goog.asserts.fail)("Group field type not supported in writeAny()");
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.MESSAGE:
	            (0, goog.asserts.fail)("Message field type not supported in writeAny()");
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.BYTES:
	            this.writeBytes(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.UINT32:
	            this.writeUint32(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.ENUM:
	            this.writeEnum(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.SFIXED32:
	            this.writeSfixed32(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.SFIXED64:
	            this.writeSfixed64(b, c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.SINT32:
	            this.writeSint32(b,
	                c);
	            break;
	        case module$contents$jspb$BinaryConstants_FieldType.SINT64:
	            this.writeSint64(b, c);
	            break;
	        default:
	            (0, goog.asserts.fail)("Invalid field type in writeAny()");
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeUnsignedVarint32_ = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeUnsignedVarint32(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSignedVarint32_ = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertSignedInteger(a, b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeSignedVarint32(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeUnsignedVarint64_ = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeUnsignedVarint64(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSignedVarint64_ = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeSignedVarint64(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeZigzagVarint32_ = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeZigzagVarint32(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeZigzagVarint64_ = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeZigzagVarint64(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeZigzagVarint64String_ = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeZigzagVarint64String(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeInt32 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, -2147483648 <= b && 2147483648 > b), this.writeSignedVarint32_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeInt32String = function (a, b) {
	    null != b && (b = parseInt(b, 10), module$contents$jspb$BinaryWriter_assertThat(a, b, -2147483648 <= b && 2147483648 > b), this.writeSignedVarint32_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeInt64 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, -9223372036854775808 <= b && 0x7fffffffffffffff > b), this.writeSignedVarint64_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeInt64String = function (a, b) {
	    null != b && (b = jspb.arith.Int64.fromString(b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeSplitVarint64(b.lo, b.hi));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeUint32 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, 0 <= b && 4294967296 > b), this.writeUnsignedVarint32_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeUint32String = function (a, b) {
	    if (null != b) {
	        var c = parseInt(b, 10);
	        module$contents$jspb$BinaryWriter_assertThat(a, b, 0 <= c && 4294967296 > c);
	        this.writeUnsignedVarint32_(a, c);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeUint64 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, 0 <= b && 1.8446744073709552E19 > b), this.writeUnsignedVarint64_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeUint64String = function (a, b) {
	    null != b && (b = jspb.arith.UInt64.fromString(b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeSplitVarint64(b.lo, b.hi));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSint32 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, -2147483648 <= b && 2147483648 > b), this.writeZigzagVarint32_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSint64 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, -9223372036854775808 <= b && 0x7fffffffffffffff > b), this.writeZigzagVarint64_(a, b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSint64String = function (a, b) {
	    null != b && this.writeZigzagVarint64String_(a, b);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeFixed32 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, 0 <= b && 4294967296 > b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED32), this.encoder_.writeUint32(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeFixed64 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, 0 <= b && 1.8446744073709552E19 > b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED64), this.encoder_.writeUint64(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeFixed64String = function (a, b) {
	    null != b && (b = jspb.arith.UInt64.fromString(b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED64), this.encoder_.writeSplitFixed64(b.lo, b.hi));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSfixed32 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, -2147483648 <= b && 2147483648 > b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED32), this.encoder_.writeInt32(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSfixed64 = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, -9223372036854775808 <= b && 0x7fffffffffffffff > b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED64), this.encoder_.writeInt64(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSfixed64String = function (a, b) {
	    null != b && (b = jspb.arith.Int64.fromString(b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED64), this.encoder_.writeSplitFixed64(b.lo, b.hi));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeFloat = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED32), this.encoder_.writeFloat(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeDouble = function (a, b) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED64), this.encoder_.writeDouble(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeBool = function (a, b) {
	    null != b && (module$contents$jspb$BinaryWriter_assertThat(a, b, "boolean" === typeof b || "number" === typeof b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeBool(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeEnum = function (a, b) {
	    null != b && (b = parseInt(b, 10), module$contents$jspb$BinaryWriter_assertSignedInteger(a, b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeSignedVarint32(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeString = function (a, b) {
	    null != b && (a = this.beginDelimited_(a), this.encoder_.writeString(b), this.endDelimited_(a));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeBytes = function (a, b) {
	    null != b && (b = module$contents$jspb$utils_byteSourceToUint8Array(b), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(b.length), this.appendUint8Array_(b));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeMessage = function (a, b, c) {
	    null != b && (a = this.beginDelimited_(a), c(b, this), this.endDelimited_(a));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeMessageSet = function (a, b, c) {
	    null != b && (this.writeFieldHeader_(1, module$contents$jspb$BinaryConstants_WireType.START_GROUP), this.writeFieldHeader_(2, module$contents$jspb$BinaryConstants_WireType.VARINT), this.encoder_.writeSignedVarint32(a), a = this.beginDelimited_(3), c(b, this), this.endDelimited_(a), this.writeFieldHeader_(1, module$contents$jspb$BinaryConstants_WireType.END_GROUP));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeGroup = function (a, b, c) {
	    null != b && (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.START_GROUP), c(b, this), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.END_GROUP));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSplitFixed64 = function (a, b, c) {
	    this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.FIXED64);
	    this.encoder_.writeSplitFixed64(b, c);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSplitVarint64 = function (a, b, c) {
	    this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT);
	    this.encoder_.writeSplitVarint64(b, c);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeSplitZigzagVarint64 = function (a, b, c) {
	    this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.VARINT);
	    this.encoder_.writeSplitZigzagVarint64(b >>> 0, c >>> 0);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedInt32 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeSignedVarint32_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedInt32String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeInt32String(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedInt64 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeSignedVarint64_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSplitFixed64 = function (a, b, c, d) {
	    if (null != b)
	        for (var e = 0; e < b.length; e++) this.writeSplitFixed64(a, c(b[e]), d(b[e]));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSplitVarint64 = function (a, b, c, d) {
	    if (null != b)
	        for (var e = 0; e < b.length; e++) this.writeSplitVarint64(a, c(b[e]), d(b[e]));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSplitZigzagVarint64 = function (a, b, c, d) {
	    if (null != b)
	        for (var e = 0; e < b.length; e++) this.writeSplitZigzagVarint64(a, c(b[e]), d(b[e]));
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedInt64String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeInt64String(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedUint32 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeUnsignedVarint32_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedUint32String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeUint32String(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedUint64 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeUnsignedVarint64_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedUint64String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeUint64String(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSint32 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeZigzagVarint32_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSint64 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeZigzagVarint64_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSint64String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeZigzagVarint64String_(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedFixed32 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeFixed32(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedFixed64 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeFixed64(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedFixed64String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeFixed64String(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSfixed32 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeSfixed32(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSfixed64 = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeSfixed64(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedSfixed64String = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeSfixed64String(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedFloat = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeFloat(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedDouble = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeDouble(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedBool = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeBool(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedEnum = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeEnum(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedString = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeString(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedBytes = function (a, b) {
	    if (null != b)
	        for (var c = 0; c < b.length; c++) this.writeBytes(a, b[c]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedMessage = function (a, b, c) {
	    if (null != b)
	        for (var d = 0; d < b.length; d++) {
	            var e = this.beginDelimited_(a);
	            c(b[d], this);
	            this.endDelimited_(e);
	        }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writeRepeatedGroup = function (a, b, c) {
	    if (null != b)
	        for (var d = 0; d < b.length; d++) this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.START_GROUP), c(b[d], this), this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.END_GROUP);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedInt32 = function (a, b) {
	    if (null != b && b.length) {
	        for (var c = this.beginDelimited_(a), d = 0; d < b.length; d++) module$contents$jspb$BinaryWriter_assertSignedInteger(a, b[d]), this.encoder_.writeSignedVarint32(b[d]);
	        this.endDelimited_(c);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedInt32String = function (a, b) {
	    if (null != b && b.length) {
	        for (var c = this.beginDelimited_(a), d = 0; d < b.length; d++) {
	            var e = parseInt(b[d], 10);
	            module$contents$jspb$BinaryWriter_assertSignedInteger(a, e);
	            this.encoder_.writeSignedVarint32(e);
	        }
	        this.endDelimited_(c);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedInt64 = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeSignedVarint64(b[c]);
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSplitFixed64 = function (a, b, c, d) {
	    if (null != b) {
	        a = this.beginDelimited_(a);
	        for (var e = 0; e < b.length; e++) this.encoder_.writeSplitFixed64(c(b[e]), d(b[e]));
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSplitVarint64 = function (a, b, c, d) {
	    if (null != b) {
	        a = this.beginDelimited_(a);
	        for (var e = 0; e < b.length; e++) this.encoder_.writeSplitVarint64(c(b[e]), d(b[e]));
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSplitZigzagVarint64 = function (a, b, c, d) {
	    if (null != b) {
	        a = this.beginDelimited_(a);
	        for (var e = this.encoder_, f = 0; f < b.length; f++) e.writeSplitZigzagVarint64(c(b[f]), d(b[f]));
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedInt64String = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) {
	            var d = jspb.arith.Int64.fromString(b[c]);
	            this.encoder_.writeSplitVarint64(d.lo, d.hi);
	        }
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedUint32 = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeUnsignedVarint32(b[c]);
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedUint32String = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeUnsignedVarint32(parseInt(b[c], 10));
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedUint64 = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeUnsignedVarint64(b[c]);
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedUint64String = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) {
	            var d = jspb.arith.UInt64.fromString(b[c]);
	            this.encoder_.writeSplitVarint64(d.lo, d.hi);
	        }
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSint32 = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeZigzagVarint32(b[c]);
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSint64 = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeZigzagVarint64(b[c]);
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSint64String = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeZigzagVarint64String(b[c]);
	        this.endDelimited_(a);
	    }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedFixed32 = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(4 * b.length), a = 0; a < b.length; a++) this.encoder_.writeUint32(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedFixed64 = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(8 * b.length), a = 0; a < b.length; a++) this.encoder_.writeUint64(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedFixed64String = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(8 * b.length), a = 0; a < b.length; a++) {
	            var c = jspb.arith.UInt64.fromString(b[a]);
	            this.encoder_.writeSplitFixed64(c.lo, c.hi);
	        }
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSfixed32 = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(4 * b.length), a = 0; a < b.length; a++) this.encoder_.writeInt32(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSfixed64 = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(8 * b.length), a = 0; a < b.length; a++) this.encoder_.writeInt64(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedSfixed64String = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(8 * b.length), a = 0; a < b.length; a++) this.encoder_.writeInt64String(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedFloat = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(4 * b.length), a = 0; a < b.length; a++) this.encoder_.writeFloat(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedDouble = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(8 * b.length), a = 0; a < b.length; a++) this.encoder_.writeDouble(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedBool = function (a, b) {
	    if (null != b && b.length)
	        for (this.writeFieldHeader_(a, module$contents$jspb$BinaryConstants_WireType.DELIMITED), this.encoder_.writeUnsignedVarint32(b.length), a = 0; a < b.length; a++) this.encoder_.writeBool(b[a]);
	};
	module$contents$jspb$BinaryWriter_BinaryWriter.prototype.writePackedEnum = function (a, b) {
	    if (null != b && b.length) {
	        a = this.beginDelimited_(a);
	        for (var c = 0; c < b.length; c++) this.encoder_.writeEnum(b[c]);
	        this.endDelimited_(a);
	    }
	};

	function module$contents$jspb$BinaryWriter_assertSignedInteger(a, b) {
	    module$contents$jspb$BinaryWriter_assertThat(a, b, b === Math.floor(b));
	    module$contents$jspb$BinaryWriter_assertThat(a, b, -2147483648 <= b && 2147483648 > b);
	}

	function module$contents$jspb$BinaryWriter_assertThat(a, b, c) {
	    c || (0, goog.asserts.fail)("for [" + b + "] at [" + a + "]");
	}
	jspb.BinaryWriter = module$contents$jspb$BinaryWriter_BinaryWriter;
	var module$contents$jspb$ExtensionFieldInfo_ExtensionFieldInfo = function (a, b, c, d, e) {
	    this.fieldIndex = a;
	    this.fieldName = b;
	    this.ctor = c;
	    this.toObjectFn = d;
	    this.isRepeated = e;
	};
	module$contents$jspb$ExtensionFieldInfo_ExtensionFieldInfo.prototype.isMessageType = function () {
	    return !!this.ctor
	};
	jspb.ExtensionFieldInfo = module$contents$jspb$ExtensionFieldInfo_ExtensionFieldInfo;
	jspb.ExtensionFieldBinaryInfo = function (a, b, c, d, e, f) {
	    this.fieldInfo = a;
	    this.binaryReaderFn = b;
	    this.binaryWriterFn = c;
	    this.binaryMessageSerializeFn = d;
	    this.binaryMessageDeserializeFn = e;
	    this.isPacked = f || !1;
	};
	var module$exports$jspb$Freezer$Loading$Info = {
	    isFreezerLoaded: !1
	};
	var module$contents$jspb$Map_Map = function (a, b) {
	    this.arr_ = a;
	    this.valueCtor = b;
	    this.map = {};
	    this.arrClean = !0;
	    this.markMessageFrozenFn_ = null;
	    0 < this.arr_.length && this.loadFromArray_();
	};
	module$contents$jspb$Map_Map.prototype.isFrozen = function () {
	    return module$exports$jspb$Freezer$Loading$Info.isFreezerLoaded 
	};
	module$contents$jspb$Map_Map.prototype.internalMarkFrozen = function (a) {
	    goog.asserts.assert(module$exports$jspb$Freezer$Loading$Info.isFreezerLoaded);
	    this.markMessageFrozenFn_ = a;
	};
	module$contents$jspb$Map_Map.prototype.checkNotFrozen_ = function () {
	};
	module$contents$jspb$Map_Map.prototype.loadFromArray_ = function () {
	    for (var a = 0; a < this.arr_.length; a++) {
	        var b = this.arr_[a],
	            c = b[0];
	        this.map[c.toString()] = new module$contents$jspb$Map_Entry_(c, b[1]);
	    }
	    this.arrClean = !0;
	};
	module$contents$jspb$Map_Map.prototype.toArray = function () {
	    this.checkNotFrozen_();
	    return this.syncInternalArray_(!1)
	};
	module$contents$jspb$Map_Map.prototype.toArrayInternal = function () {
	    return this.syncInternalArray_(!0)
	};
	module$contents$jspb$Map_Map.prototype.toArrayHelper_ = function (a, b) {
	    return a.toArray()
	};
	module$contents$jspb$Map_Map.prototype.syncInternalArray_ = function (a) {
	    if (this.arrClean) {
	        if (this.valueCtor) {
	            var b = this.map,
	                c;
	            for (c in b)
	                if (Object.prototype.hasOwnProperty.call(b, c)) {
	                    var d = b[c].valueWrapper;
	                    d && this.toArrayHelper_(d, a);
	                }
	        }
	    } else {
	        this.arr_.length = 0;
	        b = this.stringKeys_();
	        b.sort();
	        for (c = 0; c < b.length; c++) {
	            var e = this.map[b[c]];
	            (d = e.valueWrapper) && this.toArrayHelper_(d, a);
	            this.arr_.push([e.key, e.value]);
	        }
	        this.arrClean = !0;
	    }
	    return this.arr_
	};
	module$contents$jspb$Map_Map.prototype.toObject = function (a, b) {
	    var c = [];
	    this.forEach(function (d, e) {
	        c.push([e, b ? b(a, d) : d]);
	    });
	    return c
	};
	module$contents$jspb$Map_Map.fromObject = function (a, b, c) {
	    b = new module$contents$jspb$Map_Map([], b);
	    for (var d = 0; d < a.length; d++) {
	        var e = a[d][0],
	            f = c(a[d][1]);
	        b.set(e, f);
	    }
	    return b
	};
	module$contents$jspb$Map_Map.prototype.getLength = function () {
	    return this.stringKeys_().length
	};
	module$contents$jspb$Map_Map.prototype.clear = function () {
	    this.checkNotFrozen_();
	    this.map = {};
	    this.arrClean = !1;
	};
	module$contents$jspb$Map_Map.prototype.del = function (a) {
	    this.checkNotFrozen_();
	    a = a.toString();
	    var b = this.map.hasOwnProperty(a);
	    delete this.map[a];
	    this.arrClean = !1;
	    return b
	};
	module$contents$jspb$Map_Map.prototype.getEntryList = function () {
	    var a = [],
	        b = this.stringKeys_();
	    b.sort();
	    for (var c = 0; c < b.length; c++) {
	        var d = this.map[b[c]];
	        a.push([d.key, d.value]);
	    }
	    return a
	};
	module$contents$jspb$Map_Map.prototype.entries = function () {
	    var a = [],
	        b = this.stringKeys_();
	    b.sort();
	    for (var c = 0; c < b.length; c++) {
	        var d = this.map[b[c]];
	        a.push([d.key, this.wrapEntry_(d)]);
	    }
	    return new module$contents$jspb$Map_ArrayIteratorIterable(a)
	};
	module$contents$jspb$Map_Map.prototype.keys = function () {
	    var a = [],
	        b = this.stringKeys_();
	    b.sort();
	    for (var c = 0; c < b.length; c++) a.push(this.map[b[c]].key);
	    return new module$contents$jspb$Map_ArrayIteratorIterable(a)
	};
	module$contents$jspb$Map_Map.prototype.values = function () {
	    var a = [],
	        b = this.stringKeys_();
	    b.sort();
	    for (var c = 0; c < b.length; c++) a.push(this.wrapEntry_(this.map[b[c]]));
	    return new module$contents$jspb$Map_ArrayIteratorIterable(a)
	};
	module$contents$jspb$Map_Map.prototype.forEach = function (a, b) {
	    var c = this.stringKeys_();
	    c.sort();
	    for (var d = 0; d < c.length; d++) {
	        var e = this.map[c[d]];
	        a.call(b, this.wrapEntry_(e), e.key, this);
	    }
	};
	module$contents$jspb$Map_Map.prototype.set = function (a, b) {
	    this.checkNotFrozen_();
	    var c = new module$contents$jspb$Map_Entry_(a);
	    this.valueCtor ? (c.valueWrapper = b, c.value = b.toArrayInternal()) : c.value = b;
	    this.map[a.toString()] = c;
	    this.arrClean = !1;
	    return this
	};
	module$contents$jspb$Map_Map.prototype.setRawData = function (a, b) {
	    this.map[a.toString()] = new module$contents$jspb$Map_Entry_(a, b);
	    this.arrClean = !1;
	};
	module$contents$jspb$Map_Map.prototype.wrapEntry_ = function (a) {
	    return this.valueCtor ? (a.valueWrapper || (a.valueWrapper = new this.valueCtor(a.value), this.isFrozen() && (goog.asserts.assert(null != this.markMessageFrozenFn_), this.markMessageFrozenFn_(a.valueWrapper))), a.valueWrapper) : a.value
	};
	module$contents$jspb$Map_Map.prototype.get = function (a) {
	    if (a = this.map[a.toString()]) return this.wrapEntry_(a)
	};
	module$contents$jspb$Map_Map.prototype.has = function (a) {
	    return a.toString() in this.map
	};
	module$contents$jspb$Map_Map.prototype.serializeBinary = function (a, b, c, d, e) {
	    var f = this.stringKeys_();
	    f.sort();
	    for (var g = 0; g < f.length; g++) {
	        var h = this.map[f[g]];
	        b.beginSubMessage(a);
	        c.call(b, 1, h.key);
	        this.valueCtor ? d.call(b, 2, this.wrapEntry_(h), e) : d.call(b, 2, h.value);
	        b.endSubMessage();
	    }
	};
	module$contents$jspb$Map_Map.deserializeBinary = function (a, b, c, d, e, f, g) {
	    for (; b.nextField() && !b.isEndGroup();) {
	        var h = b.getFieldNumber();
	        1 == h ? f = c.call(b) : 2 == h && (a.valueCtor ? (goog.asserts.assert(e), g || (g = new a.valueCtor), d.call(b, g, e)) : g = d.call(b));
	    }
	    goog.asserts.assert(void 0 != f);
	    goog.asserts.assert(void 0 != g);
	    a.set(f, g);
	};
	module$contents$jspb$Map_Map.prototype.stringKeys_ = function () {
	    var a = this.map,
	        b = [],
	        c;
	    for (c in a) Object.prototype.hasOwnProperty.call(a, c) && b.push(c);
	    return b
	};
	var module$contents$jspb$Map_Entry_ = function (a, b) {
	        this.key = a;
	        this.value = b;
	        this.valueWrapper = void 0;
	    },
	    module$contents$jspb$Map_ArrayIteratorIterable = function (a) {
	        this.idx_ = 0;
	        this.arr_ = a;
	    };
	module$contents$jspb$Map_ArrayIteratorIterable.prototype.next = function () {
	    return this.idx_ < this.arr_.length ? {
	        done: !1,
	        value: this.arr_[this.idx_++]
	    } : {
	        done: !0,
	        value: void 0
	    }
	};
	"undefined" != typeof Symbol && "undefined" != typeof Symbol.iterator && (module$contents$jspb$Map_ArrayIteratorIterable.prototype[Symbol.iterator] = function () {
	    return this
	});
	jspb.Map = module$contents$jspb$Map_Map;
	var module$contents$jspb$Message_Message = function () {};
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT = !0;
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT = !goog.DISALLOW_TEST_ONLY_CODE;
	module$contents$jspb$Message_Message.GENERATE_TO_STRING = !0;
	module$contents$jspb$Message_Message.SERIALIZE_EMPTY_TRAILING_FIELDS = !0;
	module$contents$jspb$Message_Message.SUPPORTS_UINT8ARRAY_ = "function" == typeof Uint8Array;
	module$contents$jspb$Message_Message.prototype.getJsPbMessageId = function () {
	    return this.messageId_
	};
	module$contents$jspb$Message_Message.getIndex_ = function (a, b) {
	    return b + a.arrayIndexOffset_
	};
	module$contents$jspb$Message_Message.hiddenES6Property_ = function () {};
	module$contents$jspb$Message_Message.getFieldNumber = function (a, b) {
	    return b - a.arrayIndexOffset_
	};
	module$contents$jspb$Message_Message.initialize = function (a, b, c, d, e, f) {
	    a.wrappers_ = null;
	    b || (b = c ? [c] : []);
	    a.messageId_ = c ? String(c) : void 0;
	    a.arrayIndexOffset_ = 0 === c ? -1 : 0;
	    a.array = b;
	    module$contents$jspb$Message_Message.initPivotAndExtensionObject_(a, d);
	    a.convertedPrimitiveFields_ = {};
	    module$contents$jspb$Message_Message.SERIALIZE_EMPTY_TRAILING_FIELDS || (a.repeatedFields = e);
	    if (e)
	        for (b = 0; b < e.length; b++) c = e[b], c < a.pivot_ ? (c = module$contents$jspb$Message_Message.getIndex_(a, c), a.array[c] = a.array[c] || module$contents$jspb$Message_Message.EMPTY_LIST_SENTINEL_) :
	            (module$contents$jspb$Message_Message.maybeInitEmptyExtensionObject_(a), a.extensionObject_[c] = a.extensionObject_[c] || module$contents$jspb$Message_Message.EMPTY_LIST_SENTINEL_);
	    if (f && f.length)
	        for (b = 0; b < f.length; b++) module$contents$jspb$Message_Message.computeOneofCase(a, f[b]);
	};
	module$contents$jspb$Message_Message.EMPTY_LIST_SENTINEL_ = goog.DEBUG && Object.freeze ? Object.freeze([]) : [];
	module$contents$jspb$Message_Message.isExtensionObject = function (a) {
	    return null !== a && "object" == typeof a && !Array.isArray(a) && !(module$contents$jspb$Message_Message.SUPPORTS_UINT8ARRAY_ && a instanceof Uint8Array)
	};
	module$contents$jspb$Message_Message.initPivotAndExtensionObject_ = function (a, b) {
	    var c = a.array.length,
	        d = -1;
	    if (c && (d = c - 1, c = a.array[d], module$contents$jspb$Message_Message.isExtensionObject(c))) {
	        a.pivot_ = module$contents$jspb$Message_Message.getFieldNumber(a, d);
	        a.extensionObject_ = c;
	        return
	    } - 1 < b ? (a.pivot_ = Math.max(b, module$contents$jspb$Message_Message.getFieldNumber(a, d + 1)), a.extensionObject_ = null) : a.pivot_ = Number.MAX_VALUE;
	};
	module$contents$jspb$Message_Message.maybeInitEmptyExtensionObject_ = function (a) {
	    var b = module$contents$jspb$Message_Message.getIndex_(a, a.pivot_);
	    a.array[b] || (module$contents$jspb$Message_Message.isFrozen(a) ? (a.extensionObject_ = {}, Object.freeze(a.extensionObject_)) : a.extensionObject_ = a.array[b] = {});
	};
	module$contents$jspb$Message_Message.toObjectList = function (a, b, c) {
	    for (var d = [], e = 0; e < a.length; e++) d.push(b.call(a[e], c, a[e]));
	    return d
	};
	module$contents$jspb$Message_Message.toObjectExtension = function (a, b, c, d) {
	    for (var e in c)
	        if (module$contents$jspb$Message_hasOwnProperty(c, e)) {
	            var f = c[e],
	                g = a.getExtension(f);
	            if (null != g) {
	                var h = void 0;
	                for (h in f.fieldName)
	                    if (f.fieldName.hasOwnProperty(h)) break;
	                var k = f.toObjectFn;
	                b[h] = k ? f.isRepeated ? module$contents$jspb$Message_Message.toObjectList(g, k, d) : k(d, g) : g;
	            }
	        }
	};
	module$contents$jspb$Message_Message.serializeBinaryExtensions = function (a, b, c) {
	    for (var d in c)
	        if (module$contents$jspb$Message_hasOwnProperty(c, d)) {
	            var e = c[d],
	                f = e.fieldInfo;
	            if (!e.binaryWriterFn) throw Error("Message extension present that was generated without binary serialization support");
	            var g = a.getExtension(f);
	            if (null != g)
	                if (f.isMessageType())
	                    if (e.binaryMessageSerializeFn) e.binaryWriterFn.call(b, f.fieldIndex, g, e.binaryMessageSerializeFn);
	                    else throw Error("Message extension present holding submessage without binary support enabled, and message is being serialized to binary format");
	            else e.binaryWriterFn.call(b, f.fieldIndex, g);
	        }
	};
	module$contents$jspb$Message_Message.readBinaryExtensionMessageSet = function (a, b, c) {
	    if (1 == b.getFieldNumber() && b.getWireType() == module$contents$jspb$BinaryConstants_WireType.START_GROUP) {
	        for (var d = 0, e = null; b.nextField() && (0 != b.getWireType() || 0 != b.getFieldNumber());)
	            if (b.getWireType() == module$contents$jspb$BinaryConstants_WireType.VARINT && 2 == b.getFieldNumber()) d = b.readUint32();
	            else if (b.getWireType() == module$contents$jspb$BinaryConstants_WireType.DELIMITED && 3 == b.getFieldNumber()) e = b.readBytes();
	        else if (b.getWireType() ==
	            module$contents$jspb$BinaryConstants_WireType.END_GROUP) break;
	        else b.skipField();
	        if (1 != b.getFieldNumber() || b.getWireType() != module$contents$jspb$BinaryConstants_WireType.END_GROUP || null == e || 0 == d) throw Error("Malformed binary bytes for message set");
	        if (b = c[d]) c = b.fieldInfo, d = new c.ctor, b.binaryMessageDeserializeFn.call(d, d, new module$contents$jspb$BinaryReader_BinaryReader(e)), a.setExtension(c, d);
	    } else b.skipField();
	};
	module$contents$jspb$Message_Message.readBinaryExtension = function (a, b, c) {
	    var d = c[b.getFieldNumber()];
	    if (d) {
	        c = d.fieldInfo;
	        if (!d.binaryReaderFn) throw Error("Deserializing extension whose generated code does not support binary format");
	        if (c.isMessageType()) {
	            var e = new c.ctor;
	            d.binaryReaderFn.call(b, e, d.binaryMessageDeserializeFn);
	        } else e = d.binaryReaderFn.call(b);
	        c.isRepeated && !d.isPacked ? (b = a.getExtension(c)) ? b.push(e) : a.setExtension(c, [e]) : a.setExtension(c, e);
	    } else b.skipField();
	};
	module$contents$jspb$Message_Message.getField = function (a, b) {
	    if (b < a.pivot_) {
	        b = module$contents$jspb$Message_Message.getIndex_(a, b);
	        var c = a.array[b];
	        return c !== module$contents$jspb$Message_Message.EMPTY_LIST_SENTINEL_ || module$contents$jspb$Message_Message.isFrozen(a) ? c : a.array[b] = []
	    }
	    if (a.extensionObject_) return c = a.extensionObject_[b], c === module$contents$jspb$Message_Message.EMPTY_LIST_SENTINEL_ ? a.extensionObject_[b] = [] : c
	};
	module$contents$jspb$Message_Message.hasField = function (a, b) {
	    return null != module$contents$jspb$Message_Message.getField(a, b)
	};
	module$contents$jspb$Message_Message.getRepeatedField = function (a, b) {
	    b = module$contents$jspb$Message_Message.getField(a, b);
	    module$contents$jspb$Message_Message.isFrozen(a) && module$contents$jspb$Message_Message.internalMarkFrozen(b);
	    return b
	};
	module$contents$jspb$Message_Message.getOptionalFloatingPointField = function (a, b) {
	    a = module$contents$jspb$Message_Message.getField(a, b);
	    return null == a ? a : +a
	};
	module$contents$jspb$Message_Message.getBooleanField = function (a, b) {
	    a = module$contents$jspb$Message_Message.getField(a, b);
	    return null == a ? a : !!a
	};
	module$contents$jspb$Message_Message.getRepeatedFloatingPointField = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, b);
	    a.convertedPrimitiveFields_ || (a.convertedPrimitiveFields_ = {});
	    if (!a.convertedPrimitiveFields_[b]) {
	        for (var d = 0; d < c.length; d++) c[d] = +c[d];
	        a.convertedPrimitiveFields_[b] = !0;
	    }
	    module$contents$jspb$Message_Message.isFrozen(a) && module$contents$jspb$Message_Message.internalMarkFrozen(c);
	    return c
	};
	module$contents$jspb$Message_Message.getRepeatedBooleanField = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, b);
	    a.convertedPrimitiveFields_ || (a.convertedPrimitiveFields_ = {});
	    if (!a.convertedPrimitiveFields_[b]) {
	        for (var d = 0; d < c.length; d++) c[d] = !!c[d];
	        a.convertedPrimitiveFields_[b] = !0;
	    }
	    module$contents$jspb$Message_Message.isFrozen(a) && module$contents$jspb$Message_Message.internalMarkFrozen(c);
	    return c
	};
	module$contents$jspb$Message_Message.bytesAsB64 = function (a) {
	    if (null == a || "string" === typeof a) return a;
	    if (module$contents$jspb$Message_Message.SUPPORTS_UINT8ARRAY_ && a instanceof Uint8Array) return goog.crypt.base64.encodeByteArray(a);
	    goog.asserts.fail("Cannot coerce to b64 string: " + goog.typeOf(a));
	    return null
	};
	module$contents$jspb$Message_Message.bytesAsU8 = function (a) {
	    if (null == a || a instanceof Uint8Array) return a;
	    if ("string" === typeof a) return goog.crypt.base64.decodeStringToUint8Array(a);
	    goog.asserts.fail("Cannot coerce to Uint8Array: " + goog.typeOf(a));
	    return null
	};
	module$contents$jspb$Message_Message.bytesListAsB64 = function (a) {
	    module$contents$jspb$Message_Message.assertConsistentTypes_(a);
	    return a.length && "string" !== typeof a[0] ? module$contents$goog$array_map(a, module$contents$jspb$Message_Message.bytesAsB64) : a
	};
	module$contents$jspb$Message_Message.bytesListAsU8 = function (a) {
	    module$contents$jspb$Message_Message.assertConsistentTypes_(a);
	    return !a.length || a[0] instanceof Uint8Array ? a : module$contents$goog$array_map(a, module$contents$jspb$Message_Message.bytesAsU8)
	};
	module$contents$jspb$Message_Message.assertConsistentTypes_ = function (a) {
	    if (goog.DEBUG && a && 1 < a.length) {
	        var b = goog.typeOf(a[0]);
	        module$contents$goog$array_forEach(a, function (c) {
	            goog.typeOf(c) != b && goog.asserts.fail("Inconsistent type in JSPB repeated field array. Got " + goog.typeOf(c) + " expected " + b);
	        });
	    }
	};
	module$contents$jspb$Message_Message.getFieldWithDefault = function (a, b, c) {
	    a = module$contents$jspb$Message_Message.getField(a, b);
	    return null == a ? c : a
	};
	module$contents$jspb$Message_Message.getIntegerFieldWithDefault = function (a, b, c) {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(a, b, void 0 === c ? 0 : c)
	};
	module$contents$jspb$Message_Message.getStringFieldWithDefault = function (a, b, c) {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(a, b, void 0 === c ? "" : c)
	};
	module$contents$jspb$Message_Message.getIntegerInStringFieldWithDefault = function (a, b, c) {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(a, b, void 0 === c ? "0" : c)
	};
	module$contents$jspb$Message_Message.getBooleanFieldWithDefault = function (a, b, c) {
	    c = void 0 === c ? !1 : c;
	    a = module$contents$jspb$Message_Message.getBooleanField(a, b);
	    return null == a ? c : a
	};
	module$contents$jspb$Message_Message.getFloatingPointFieldWithDefault = function (a, b, c) {
	    c = void 0 === c ? 0 : c;
	    a = module$contents$jspb$Message_Message.getOptionalFloatingPointField(a, b);
	    return null == a ? c : a
	};
	module$contents$jspb$Message_Message.getFieldProto3 = module$contents$jspb$Message_Message.getFieldWithDefault;
	module$contents$jspb$Message_Message.getMapField = function (a, b, c, d) {
	    a.wrappers_ || (a.wrappers_ = {});
	    if (b in a.wrappers_) return a.wrappers_[b];
	    var e = module$contents$jspb$Message_Message.getField(a, b);
	    if (!e) {
	        if (c) return;
	        e = [];
	        module$contents$jspb$Message_Message.isFrozen(a) || module$contents$jspb$Message_Message.setField(a, b, e);
	    }
	    c = new module$contents$jspb$Map_Map(e, d);
	    module$contents$jspb$Message_Message.isFrozen(a) && c.internalMarkFrozen(module$contents$jspb$Message_Message.internalMarkFrozen);
	    return a.wrappers_[b] =
	        c
	};
	module$contents$jspb$Message_Message.setField = function (a, b, c) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    b < a.pivot_ ? a.array[module$contents$jspb$Message_Message.getIndex_(a, b)] = c : (module$contents$jspb$Message_Message.maybeInitEmptyExtensionObject_(a), a.extensionObject_[b] = c);
	    return a
	};
	module$contents$jspb$Message_Message.clearField = function (a, b) {
	    return module$contents$jspb$Message_Message.setField(a, b, void 0)
	};
	module$contents$jspb$Message_Message.clearRepeatedField = function (a, b) {
	    return module$contents$jspb$Message_Message.setField(a, b, [])
	};
	module$contents$jspb$Message_Message.clearWrapperField = function (a, b) {
	    return module$contents$jspb$Message_Message.setWrapperField(a, b, void 0)
	};
	module$contents$jspb$Message_Message.clearRepeatedWrapperField = function (a, b) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(a, b, [])
	};
	module$contents$jspb$Message_Message.clearOneofField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setOneofField(a, b, c, void 0)
	};
	module$contents$jspb$Message_Message.clearOneofWrapperField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setOneofWrapperField(a, b, c, void 0)
	};
	module$contents$jspb$Message_Message.setProto3IntField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, 0)
	};
	module$contents$jspb$Message_Message.setProto3FloatField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, 0)
	};
	module$contents$jspb$Message_Message.setProto3BooleanField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, !1)
	};
	module$contents$jspb$Message_Message.setProto3StringField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, "")
	};
	module$contents$jspb$Message_Message.setProto3BytesField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, "")
	};
	module$contents$jspb$Message_Message.setProto3EnumField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, 0)
	};
	module$contents$jspb$Message_Message.setProto3StringIntField = function (a, b, c) {
	    return module$contents$jspb$Message_Message.setFieldIgnoringDefault_(a, b, c, "0")
	};
	module$contents$jspb$Message_Message.setFieldIgnoringDefault_ = function (a, b, c, d) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    c !== d ? module$contents$jspb$Message_Message.setField(a, b, c) : b < a.pivot_ ? a.array[module$contents$jspb$Message_Message.getIndex_(a, b)] = null : (module$contents$jspb$Message_Message.maybeInitEmptyExtensionObject_(a), delete a.extensionObject_[b]);
	    return a
	};
	module$contents$jspb$Message_Message.addToRepeatedField = function (a, b, c, d) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    b = module$contents$jspb$Message_Message.getRepeatedField(a, b);
	    void 0 != d ? b.splice(d, 0, c) : b.push(c);
	    return a
	};
	module$contents$jspb$Message_Message.setOneofField = function (a, b, c, d) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    (c = module$contents$jspb$Message_Message.computeOneofCase(a, c)) && c !== b && void 0 !== d && (a.wrappers_ && c in a.wrappers_ && (a.wrappers_[c] = void 0), module$contents$jspb$Message_Message.setField(a, c, void 0));
	    return module$contents$jspb$Message_Message.setField(a, b, d)
	};
	module$contents$jspb$Message_Message.computeOneofCase = function (a, b) {
	    for (var c, d, e = module$contents$jspb$Message_Message.isFrozen(a), f = 0; f < b.length; f++) {
	        var g = b[f],
	            h = module$contents$jspb$Message_Message.getField(a, g);
	        null != h && (c = g, d = h, e || module$contents$jspb$Message_Message.setField(a, g, void 0));
	    }
	    return c ? (e || module$contents$jspb$Message_Message.setField(a, c, d), c) : 0
	};
	module$contents$jspb$Message_Message.getWrapperField = function (a, b, c, d) {
	    a.wrappers_ || (a.wrappers_ = {});
	    if (!a.wrappers_[c]) {
	        var e = module$contents$jspb$Message_Message.getField(a, c);
	        if (d || e) a.wrappers_[c] = new b(e), module$contents$jspb$Message_Message.isFrozen(a) && module$contents$jspb$Message_Message.internalMarkFrozen(a.wrappers_[c]);
	    }
	    return a.wrappers_[c]
	};
	module$contents$jspb$Message_Message.getRepeatedWrapperField = function (a, b, c) {
	    module$contents$jspb$Message_Message.wrapRepeatedField_(a, b, c);
	    b = a.wrappers_[c];
	    b == module$contents$jspb$Message_Message.EMPTY_LIST_SENTINEL_ && (b = a.wrappers_[c] = []);
	    return b
	};
	module$contents$jspb$Message_Message.wrapRepeatedField_ = function (a, b, c) {
	    a.wrappers_ || (a.wrappers_ = {});
	    if (!a.wrappers_[c]) {
	        for (var d = module$contents$jspb$Message_Message.getRepeatedField(a, c), e = [], f = 0; f < d.length; f++) e[f] = new b(d[f]), module$contents$jspb$Message_Message.isFrozen(a) && module$contents$jspb$Message_Message.internalMarkFrozen(e[f]);
	        module$contents$jspb$Message_Message.isFrozen(a) && module$contents$jspb$Message_Message.internalMarkFrozen(e);
	        a.wrappers_[c] = e;
	    }
	};
	module$contents$jspb$Message_Message.setWrapperField = function (a, b, c) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    a.wrappers_ || (a.wrappers_ = {});
	    var d = c ? module$contents$jspb$Message_Message.toArrayHelper_(c, !0) : c;
	    a.wrappers_[b] = c;
	    return module$contents$jspb$Message_Message.setField(a, b, d)
	};
	module$contents$jspb$Message_Message.setOneofWrapperField = function (a, b, c, d) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    a.wrappers_ || (a.wrappers_ = {});
	    var e = d ? module$contents$jspb$Message_Message.toArrayHelper_(d, !0) : d;
	    a.wrappers_[b] = d;
	    return module$contents$jspb$Message_Message.setOneofField(a, b, c, e)
	};
	module$contents$jspb$Message_Message.setRepeatedWrapperField = function (a, b, c) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    a.wrappers_ || (a.wrappers_ = {});
	    c = c || [];
	    for (var d = [], e = 0; e < c.length; e++) d[e] = module$contents$jspb$Message_Message.toArrayHelper_(c[e], !0);
	    a.wrappers_[b] = c;
	    return module$contents$jspb$Message_Message.setField(a, b, d)
	};
	module$contents$jspb$Message_Message.addToRepeatedWrapperField = function (a, b, c, d, e) {
	    module$contents$jspb$Message_Message.checkNotFrozen_(a);
	    var f = module$contents$jspb$Message_Message.getRepeatedWrapperField(a, d, b);
	    c = c ? c : new d;
	    a = module$contents$jspb$Message_Message.getRepeatedField(a, b);
	    void 0 != e ? (f.splice(e, 0, c), a.splice(e, 0, module$contents$jspb$Message_Message.toArrayHelper_(c, !0))) : (f.push(c), a.push(module$contents$jspb$Message_Message.toArrayHelper_(c, !0)));
	    return c
	};
	module$contents$jspb$Message_Message.toMap = function (a, b, c, d) {
	    for (var e = {}, f = 0; f < a.length; f++) e[b.call(a[f])] = c ? c.call(a[f], d, a[f]) : a[f];
	    return e
	};
	module$contents$jspb$Message_Message.prototype.syncMapFields_ = function (a) {
	    if (this.wrappers_)
	        for (var b in this.wrappers_)
	            if (module$contents$jspb$Message_hasOwnProperty(this.wrappers_, b)) {
	                var c = this.wrappers_[b];
	                if (Array.isArray(c))
	                    for (var d = 0; d < c.length; d++) c[d] && module$contents$jspb$Message_Message.toArrayHelper_(c[d], a);
	                else c && module$contents$jspb$Message_Message.toArrayHelper_(c, a);
	            }
	};
	module$contents$jspb$Message_Message.toArrayHelper_ = function (a, b) {
	    return a.toArray()
	};
	module$contents$jspb$Message_Message.prototype.toArray = function () {
	    module$contents$jspb$Message_Message.checkNotFrozen_(this);
	    this.syncMapFields_(!1);
	    return this.array
	};
	module$contents$jspb$Message_Message.prototype.toArrayInternal = function () {
	    this.syncMapFields_(!0);
	    return this.array
	};
	module$contents$jspb$Message_Message.prototype.serialize = module$contents$jspb$Message_Message.SUPPORTS_UINT8ARRAY_ ? function () {
	    var a = Uint8Array.prototype.toJSON;
	    Uint8Array.prototype.toJSON = function () {
	        return goog.crypt.base64.encodeByteArray(this)
	    };
	    try {
	        return JSON.stringify(this.array && module$contents$jspb$Message_Message.prepareForSerialize_(module$contents$jspb$Message_Message.toArrayHelper_(this, !0), this), module$contents$jspb$Message_Message.serializeSpecialNumbers_)
	    } finally {
	        Uint8Array.prototype.toJSON =
	            a;
	    }
	} : function () {
	    return JSON.stringify(this.array && module$contents$jspb$Message_Message.prepareForSerialize_(module$contents$jspb$Message_Message.toArrayHelper_(this, !0), this), module$contents$jspb$Message_Message.serializeSpecialNumbers_)
	};
	module$contents$jspb$Message_Message.prepareForSerialize_ = function (a, b) {
	    if (module$contents$jspb$Message_Message.SERIALIZE_EMPTY_TRAILING_FIELDS) return a;
	    for (var c, d = a.length, e = !1, f, g = a.length; g--;) {
	        var h = a[g];
	        if (Array.isArray(h)) h = module$contents$jspb$Message_Message.prepareForSerialize_(h, Array.isArray(b) ? b[g] : b && b.wrappers_ ? b.wrappers_[module$contents$jspb$Message_Message.getFieldNumber(b, g)] : void 0), !h.length && b && (Array.isArray(b) || b.repeatedFields && -1 != b.repeatedFields.indexOf(module$contents$jspb$Message_Message.getFieldNumber(b,
	            g)) && (h = null)), h != a[g] && (e = !0);
	        else if (module$contents$jspb$Message_Message.isExtensionObject(h)) {
	            f = module$contents$jspb$Message_Message.prepareExtensionForSerialize_(h, b && goog.asserts.assertInstanceof(b, module$contents$jspb$Message_Message));
	            f != h && (e = !0);
	            d--;
	            continue
	        }
	        null == h && d == g + 1 ? (e = !0, d--) : e && (c || (c = a.slice(0, d)), c[g] = h);
	    }
	    if (!e) return a;
	    c || (c = a.slice(0, d));
	    f && c.push(f);
	    return c
	};
	module$contents$jspb$Message_Message.prepareExtensionForSerialize_ = function (a, b) {
	    var c = {},
	        d = !1,
	        e;
	    for (e in a)
	        if (module$contents$jspb$Message_hasOwnProperty(a, e)) {
	            var f = a[e];
	            if (Array.isArray(f)) {
	                var g = module$contents$jspb$Message_Message.prepareForSerialize_(f, b && b.wrappers_ && b.wrappers_[e]);
	                !g.length && b && b.repeatedFields && -1 != b.repeatedFields.indexOf(+e) || (c[e] = g);
	                c[e] != f && (d = !0);
	            } else null != f ? c[e] = f : d = !0;
	        } if (!d) return a;
	    for (e in c) return c;
	    return null
	};
	module$contents$jspb$Message_Message.serializeSpecialNumbers_ = function (a, b) {
	    return "number" !== typeof b || !isNaN(b) && Infinity !== b && -Infinity !== b ? b : String(b)
	};
	module$contents$jspb$Message_Message.deserializeWithCtor = function (a, b) {
	    a = new a(b ? JSON.parse(b) : null);
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    return a
	};
	module$contents$jspb$Message_Message.GENERATE_TO_STRING && (module$contents$jspb$Message_Message.prototype.toString = function () {
	    return module$contents$jspb$Message_Message.toArrayHelper_(this, !0).toString()
	});
	module$contents$jspb$Message_Message.prototype.getExtension = function (a) {
	    module$contents$jspb$Message_Message.maybeInitEmptyExtensionObject_(this);
	    this.wrappers_ || (this.wrappers_ = {});
	    var b = module$contents$jspb$Message_Message.isFrozen(this),
	        c = a.fieldIndex;
	    return a.isRepeated ? a.isMessageType() ? (this.wrappers_[c] || (this.wrappers_[c] = module$contents$goog$array_map(this.extensionObject_[c] || [], function (d) {
	            d = new a.ctor(d);
	            b && module$contents$jspb$Message_Message.internalMarkFrozen(d);
	            return d
	        })), b && module$contents$jspb$Message_Message.internalMarkFrozen(this.wrappers_[c]),
	        this.wrappers_[c]) : b ? (c = this.extensionObject_[c], c || (c = [], module$contents$jspb$Message_Message.internalMarkFrozen(c)), c) : this.extensionObject_[c] = this.extensionObject_[c] || [] : a.isMessageType() ? (!this.wrappers_[c] && this.extensionObject_[c] && (this.wrappers_[c] = new a.ctor(this.extensionObject_[c]), b && module$contents$jspb$Message_Message.internalMarkFrozen(this.wrappers_[c])), this.wrappers_[c]) : this.extensionObject_[c]
	};
	module$contents$jspb$Message_Message.prototype.setExtension = function (a, b) {
	    module$contents$jspb$Message_Message.checkNotFrozen_(this);
	    this.wrappers_ || (this.wrappers_ = {});
	    module$contents$jspb$Message_Message.maybeInitEmptyExtensionObject_(this);
	    var c = a.fieldIndex;
	    a.isRepeated ? (b = b || [], a.isMessageType() ? (this.wrappers_[c] = b, this.extensionObject_[c] = module$contents$goog$array_map(b, function (d) {
	            return module$contents$jspb$Message_Message.toArrayHelper_(d, !0)
	        })) : this.extensionObject_[c] = b) : a.isMessageType() ?
	        (this.wrappers_[c] = b, this.extensionObject_[c] = b ? module$contents$jspb$Message_Message.toArrayHelper_(b, !0) : b) : this.extensionObject_[c] = b;
	    return this
	};
	module$contents$jspb$Message_Message.difference = function (a, b) {
	    if (!(a instanceof b.constructor)) throw Error("Messages have different types.");
	    var c = module$contents$jspb$Message_Message.toArrayHelper_(a, !0);
	    b = module$contents$jspb$Message_Message.toArrayHelper_(b, !0);
	    var d = [],
	        e = 0,
	        f = c.length > b.length ? c.length : b.length;
	    a.getJsPbMessageId() && (d[0] = a.getJsPbMessageId(), e = 1);
	    for (; e < f; e++) module$contents$jspb$Message_Message.compareFields(c[e], b[e]) || (d[e] = b[e]);
	    return new a.constructor(d)
	};
	module$contents$jspb$Message_Message.equals = function (a, b) {
	    return a == b || !(!a || !b) && a instanceof b.constructor && module$contents$jspb$Message_Message.compareFields(module$contents$jspb$Message_Message.toArrayHelper_(a, !0), module$contents$jspb$Message_Message.toArrayHelper_(b, !0))
	};
	module$contents$jspb$Message_Message.compareExtensions = function (a, b) {
	    a = a || {};
	    b = b || {};
	    var c = {},
	        d;
	    for (d in a) module$contents$jspb$Message_hasOwnProperty(a, d) && (c[d] = 0);
	    for (d in b) module$contents$jspb$Message_hasOwnProperty(b, d) && (c[d] = 0);
	    for (d in c)
	        if (module$contents$jspb$Message_hasOwnProperty(c, d) && !module$contents$jspb$Message_Message.compareFields(a[d], b[d])) return !1;
	    return !0
	};
	module$contents$jspb$Message_Message.compareFields = function (a, b) {
	    if (a == b) return !0;
	    if (!goog.isObject(a) || !goog.isObject(b)) return "number" === typeof a && isNaN(a) || "number" === typeof b && isNaN(b) ? String(a) == String(b) : !1;
	    if (a.constructor != b.constructor) return !1;
	    if (module$contents$jspb$Message_Message.SUPPORTS_UINT8ARRAY_ && a.constructor === Uint8Array) {
	        if (a.length != b.length) return !1;
	        for (var c = 0; c < a.length; c++)
	            if (a[c] != b[c]) return !1;
	        return !0
	    }
	    if (a.constructor === Array) {
	        var d = void 0,
	            e = void 0,
	            f = Math.max(a.length,
	                b.length);
	        for (c = 0; c < f; c++) {
	            var g = a[c],
	                h = b[c];
	            g && g.constructor == Object && (goog.asserts.assert(void 0 === d), goog.asserts.assert(c === a.length - 1), d = g, g = void 0);
	            h && h.constructor == Object && (goog.asserts.assert(void 0 === e), goog.asserts.assert(c === b.length - 1), e = h, h = void 0);
	            if (!module$contents$jspb$Message_Message.compareFields(g, h)) return !1
	        }
	        return d || e ? (d = d || {}, e = e || {}, module$contents$jspb$Message_Message.compareExtensions(d, e)) : !0
	    }
	    if (a.constructor === Object) return module$contents$jspb$Message_Message.compareExtensions(a,
	        b);
	    throw Error("Invalid type in JSPB array");
	};
	module$contents$jspb$Message_Message.prototype.cloneMessage = function () {
	    return module$contents$jspb$Message_Message.cloneMessage(this)
	};
	module$contents$jspb$Message_Message.prototype.clone = function () {
	    return module$contents$jspb$Message_Message.cloneMessage(this)
	};
	module$contents$jspb$Message_Message.clone = function (a) {
	    return module$contents$jspb$Message_Message.cloneMessage(a)
	};
	module$contents$jspb$Message_Message.cloneMessage = function (a) {
	    var b = module$contents$jspb$Message_Message.clone_(module$contents$jspb$Message_Message.toArrayHelper_(a, !0)),
	        c = module$contents$jspb$Message_Message.initialize;
	    module$contents$jspb$Message_Message.initialize = function (d, e, f, g, h, k) {
	        c(d, b, f, g, h, k);
	        module$contents$jspb$Message_Message.initialize = c;
	    };
	    a = new a.constructor(b);
	    module$contents$jspb$Message_Message.initialize !== c && (module$contents$jspb$Message_Message.initialize = c);
	    return a
	};
	module$contents$jspb$Message_Message.copyInto = function (a, b) {
	    goog.asserts.assertInstanceof(a, module$contents$jspb$Message_Message);
	    goog.asserts.assertInstanceof(b, module$contents$jspb$Message_Message);
	    goog.asserts.assert(a.constructor == b.constructor, "Copy source and target message should have the same type.");
	    a = module$contents$jspb$Message_Message.clone(a);
	    for (var c = module$contents$jspb$Message_Message.toArrayHelper_(b, !0), d = module$contents$jspb$Message_Message.toArrayHelper_(a, !0), e = c.length = 0; e <
	        d.length; e++) c[e] = d[e];
	    b.wrappers_ = a.wrappers_;
	    b.extensionObject_ = a.extensionObject_;
	};
	module$contents$jspb$Message_Message.clone_ = function (a) {
	    if (Array.isArray(a)) {
	        for (var b = Array(a.length), c = 0; c < a.length; c++) {
	            var d = a[c];
	            null != d && (b[c] = "object" == typeof d ? module$contents$jspb$Message_Message.clone_(goog.asserts.assert(d)) : d);
	        }
	        return b
	    }
	    if (module$contents$jspb$Message_Message.SUPPORTS_UINT8ARRAY_ && a instanceof Uint8Array) return new Uint8Array(a);
	    b = {};
	    for (c in a) module$contents$jspb$Message_hasOwnProperty(a, c) && (d = a[c], null != d && (b[c] = "object" == typeof d ? module$contents$jspb$Message_Message.clone_(goog.asserts.assert(d)) :
	        d));
	    return b
	};
	module$contents$jspb$Message_Message.messageSetExtensions = {};
	module$contents$jspb$Message_Message.messageSetExtensionsBinary = {};
	module$contents$jspb$Message_Message.isFrozen = function (a) {
	    return !1
	};
	module$contents$jspb$Message_Message.internalMarkFrozen = function (a) {
	    goog.asserts.assert(module$exports$jspb$Freezer$Loading$Info.isFreezerLoaded);
	    Array.isArray(a) ? Object.freeze(a) : (Object.freeze(a.array), a.extensionObject_ && Object.freeze(a.extensionObject_));
	};
	module$contents$jspb$Message_Message.checkNotFrozen_ = function (a) {
	};

	function module$contents$jspb$Message_hasOwnProperty(a, b) {
	    return goog.TRUSTED_SITE || Object.prototype.hasOwnProperty.call(a, b)
	}
	jspb.Message = module$contents$jspb$Message_Message;
	goog.uri = {};
	goog.uri.utils = {};
	goog.uri.utils.CharCode_ = {
	    AMPERSAND: 38,
	    EQUAL: 61,
	    HASH: 35,
	    QUESTION: 63
	};
	goog.uri.utils.buildFromEncodedParts = function (a, b, c, d, e, f, g) {
	    var h = "";
	    a && (h += a + ":");
	    c && (h += "//", b && (h += b + "@"), h += c, d && (h += ":" + d));
	    e && (h += e);
	    f && (h += "?" + f);
	    g && (h += "#" + g);
	    return h
	};
	goog.uri.utils.splitRe_ = /^(?:([^:/?#.]+):)?(?:\/\/(?:([^\\/?#]*)@)?([^\\/?#]*?)(?::([0-9]+))?(?=[\\/?#]|$))?([^?#]+)?(?:\?([^#]*))?(?:#([\s\S]*))?$/;
	goog.uri.utils.ComponentIndex = {
	    SCHEME: 1,
	    USER_INFO: 2,
	    DOMAIN: 3,
	    PORT: 4,
	    PATH: 5,
	    QUERY_DATA: 6,
	    FRAGMENT: 7
	};
	goog.uri.utils.urlPackageSupportLoggingHandler_ = null;
	goog.uri.utils.setUrlPackageSupportLoggingHandler = function (a) {
	    goog.uri.utils.urlPackageSupportLoggingHandler_ = a;
	};
	goog.uri.utils.split = function (a) {
	    var b = a.match(goog.uri.utils.splitRe_);
	    goog.uri.utils.urlPackageSupportLoggingHandler_ && 0 <= ["http", "https", "ws", "wss", "ftp"].indexOf(b[goog.uri.utils.ComponentIndex.SCHEME]) && goog.uri.utils.urlPackageSupportLoggingHandler_(a);
	    return b
	};
	goog.uri.utils.decodeIfPossible_ = function (a, b) {
	    return a ? b ? decodeURI(a) : decodeURIComponent(a) : a
	};
	goog.uri.utils.getComponentByIndex_ = function (a, b) {
	    return goog.uri.utils.split(b)[a] || null
	};
	goog.uri.utils.getScheme = function (a) {
	    return goog.uri.utils.getComponentByIndex_(goog.uri.utils.ComponentIndex.SCHEME, a)
	};
	goog.uri.utils.getEffectiveScheme = function (a) {
	    a = goog.uri.utils.getScheme(a);
	    !a && goog.global.self && goog.global.self.location && (a = goog.global.self.location.protocol, a = a.substr(0, a.length - 1));
	    return a ? a.toLowerCase() : ""
	};
	goog.uri.utils.getUserInfoEncoded = function (a) {
	    return goog.uri.utils.getComponentByIndex_(goog.uri.utils.ComponentIndex.USER_INFO, a)
	};
	goog.uri.utils.getUserInfo = function (a) {
	    return goog.uri.utils.decodeIfPossible_(goog.uri.utils.getUserInfoEncoded(a))
	};
	goog.uri.utils.getDomainEncoded = function (a) {
	    return goog.uri.utils.getComponentByIndex_(goog.uri.utils.ComponentIndex.DOMAIN, a)
	};
	goog.uri.utils.getDomain = function (a) {
	    return goog.uri.utils.decodeIfPossible_(goog.uri.utils.getDomainEncoded(a), !0)
	};
	goog.uri.utils.getPort = function (a) {
	    return Number(goog.uri.utils.getComponentByIndex_(goog.uri.utils.ComponentIndex.PORT, a)) || null
	};
	goog.uri.utils.getPathEncoded = function (a) {
	    return goog.uri.utils.getComponentByIndex_(goog.uri.utils.ComponentIndex.PATH, a)
	};
	goog.uri.utils.getPath = function (a) {
	    return goog.uri.utils.decodeIfPossible_(goog.uri.utils.getPathEncoded(a), !0)
	};
	goog.uri.utils.getQueryData = function (a) {
	    return goog.uri.utils.getComponentByIndex_(goog.uri.utils.ComponentIndex.QUERY_DATA, a)
	};
	goog.uri.utils.getFragmentEncoded = function (a) {
	    var b = a.indexOf("#");
	    return 0 > b ? null : a.substr(b + 1)
	};
	goog.uri.utils.setFragmentEncoded = function (a, b) {
	    return goog.uri.utils.removeFragment(a) + (b ? "#" + b : "")
	};
	goog.uri.utils.getFragment = function (a) {
	    return goog.uri.utils.decodeIfPossible_(goog.uri.utils.getFragmentEncoded(a))
	};
	goog.uri.utils.getHost = function (a) {
	    a = goog.uri.utils.split(a);
	    return goog.uri.utils.buildFromEncodedParts(a[goog.uri.utils.ComponentIndex.SCHEME], a[goog.uri.utils.ComponentIndex.USER_INFO], a[goog.uri.utils.ComponentIndex.DOMAIN], a[goog.uri.utils.ComponentIndex.PORT])
	};
	goog.uri.utils.getOrigin = function (a) {
	    a = goog.uri.utils.split(a);
	    return goog.uri.utils.buildFromEncodedParts(a[goog.uri.utils.ComponentIndex.SCHEME], null, a[goog.uri.utils.ComponentIndex.DOMAIN], a[goog.uri.utils.ComponentIndex.PORT])
	};
	goog.uri.utils.getPathAndAfter = function (a) {
	    a = goog.uri.utils.split(a);
	    return goog.uri.utils.buildFromEncodedParts(null, null, null, null, a[goog.uri.utils.ComponentIndex.PATH], a[goog.uri.utils.ComponentIndex.QUERY_DATA], a[goog.uri.utils.ComponentIndex.FRAGMENT])
	};
	goog.uri.utils.removeFragment = function (a) {
	    var b = a.indexOf("#");
	    return 0 > b ? a : a.substr(0, b)
	};
	goog.uri.utils.haveSameDomain = function (a, b) {
	    a = goog.uri.utils.split(a);
	    b = goog.uri.utils.split(b);
	    return a[goog.uri.utils.ComponentIndex.DOMAIN] == b[goog.uri.utils.ComponentIndex.DOMAIN] && a[goog.uri.utils.ComponentIndex.SCHEME] == b[goog.uri.utils.ComponentIndex.SCHEME] && a[goog.uri.utils.ComponentIndex.PORT] == b[goog.uri.utils.ComponentIndex.PORT]
	};
	goog.uri.utils.assertNoFragmentsOrQueries_ = function (a) {
	    goog.asserts.assert(0 > a.indexOf("#") && 0 > a.indexOf("?"), "goog.uri.utils: Fragment or query identifiers are not supported: [%s]", a);
	};
	goog.uri.utils.parseQueryData = function (a, b) {
	    if (a) {
	        a = a.split("&");
	        for (var c = 0; c < a.length; c++) {
	            var d = a[c].indexOf("="),
	                e = null;
	            if (0 <= d) {
	                var f = a[c].substring(0, d);
	                e = a[c].substring(d + 1);
	            } else f = a[c];
	            b(f, e ? goog.string.urlDecode(e) : "");
	        }
	    }
	};
	goog.uri.utils.splitQueryData_ = function (a) {
	    var b = a.indexOf("#");
	    0 > b && (b = a.length);
	    var c = a.indexOf("?");
	    if (0 > c || c > b) {
	        c = b;
	        var d = "";
	    } else d = a.substring(c + 1, b);
	    return [a.substr(0, c), d, a.substr(b)]
	};
	goog.uri.utils.joinQueryData_ = function (a) {
	    return a[0] + (a[1] ? "?" + a[1] : "") + a[2]
	};
	goog.uri.utils.appendQueryData_ = function (a, b) {
	    return b ? a ? a + "&" + b : b : a
	};
	goog.uri.utils.appendQueryDataToUri_ = function (a, b) {
	    if (!b) return a;
	    a = goog.uri.utils.splitQueryData_(a);
	    a[1] = goog.uri.utils.appendQueryData_(a[1], b);
	    return goog.uri.utils.joinQueryData_(a)
	};
	goog.uri.utils.appendKeyValuePairs_ = function (a, b, c) {
	    goog.asserts.assertString(a);
	    if (Array.isArray(b)) {
	        goog.asserts.assertArray(b);
	        for (var d = 0; d < b.length; d++) goog.uri.utils.appendKeyValuePairs_(a, String(b[d]), c);
	    } else null != b && c.push(a + ("" === b ? "" : "=" + goog.string.urlEncode(b)));
	};
	goog.uri.utils.buildQueryData = function (a, b) {
	    goog.asserts.assert(0 == Math.max(a.length - (b || 0), 0) % 2, "goog.uri.utils: Key/value lists must be even in length.");
	    var c = [];
	    for (b = b || 0; b < a.length; b += 2) goog.uri.utils.appendKeyValuePairs_(a[b], a[b + 1], c);
	    return c.join("&")
	};
	goog.uri.utils.buildQueryDataFromMap = function (a) {
	    var b = [],
	        c;
	    for (c in a) goog.uri.utils.appendKeyValuePairs_(c, a[c], b);
	    return b.join("&")
	};
	goog.uri.utils.appendParams = function (a, b) {
	    var c = 2 == arguments.length ? goog.uri.utils.buildQueryData(arguments[1], 0) : goog.uri.utils.buildQueryData(arguments, 1);
	    return goog.uri.utils.appendQueryDataToUri_(a, c)
	};
	goog.uri.utils.appendParamsFromMap = function (a, b) {
	    b = goog.uri.utils.buildQueryDataFromMap(b);
	    return goog.uri.utils.appendQueryDataToUri_(a, b)
	};
	goog.uri.utils.appendParam = function (a, b, c) {
	    c = null != c ? "=" + goog.string.urlEncode(c) : "";
	    return goog.uri.utils.appendQueryDataToUri_(a, b + c)
	};
	goog.uri.utils.findParam_ = function (a, b, c, d) {
	    for (var e = c.length; 0 <= (b = a.indexOf(c, b)) && b < d;) {
	        var f = a.charCodeAt(b - 1);
	        if (f == goog.uri.utils.CharCode_.AMPERSAND || f == goog.uri.utils.CharCode_.QUESTION)
	            if (f = a.charCodeAt(b + e), !f || f == goog.uri.utils.CharCode_.EQUAL || f == goog.uri.utils.CharCode_.AMPERSAND || f == goog.uri.utils.CharCode_.HASH) return b;
	        b += e + 1;
	    }
	    return -1
	};
	goog.uri.utils.hashOrEndRe_ = /#|$/;
	goog.uri.utils.hasParam = function (a, b) {
	    return 0 <= goog.uri.utils.findParam_(a, 0, b, a.search(goog.uri.utils.hashOrEndRe_))
	};
	goog.uri.utils.getParamValue = function (a, b) {
	    var c = a.search(goog.uri.utils.hashOrEndRe_),
	        d = goog.uri.utils.findParam_(a, 0, b, c);
	    if (0 > d) return null;
	    var e = a.indexOf("&", d);
	    if (0 > e || e > c) e = c;
	    d += b.length + 1;
	    return goog.string.urlDecode(a.substr(d, e - d))
	};
	goog.uri.utils.getParamValues = function (a, b) {
	    for (var c = a.search(goog.uri.utils.hashOrEndRe_), d = 0, e, f = []; 0 <= (e = goog.uri.utils.findParam_(a, d, b, c));) {
	        d = a.indexOf("&", e);
	        if (0 > d || d > c) d = c;
	        e += b.length + 1;
	        f.push(goog.string.urlDecode(a.substr(e, d - e)));
	    }
	    return f
	};
	goog.uri.utils.trailingQueryPunctuationRe_ = /[?&]($|#)/;
	goog.uri.utils.removeParam = function (a, b) {
	    for (var c = a.search(goog.uri.utils.hashOrEndRe_), d = 0, e, f = []; 0 <= (e = goog.uri.utils.findParam_(a, d, b, c));) f.push(a.substring(d, e)), d = Math.min(a.indexOf("&", e) + 1 || c, c);
	    f.push(a.substr(d));
	    return f.join("").replace(goog.uri.utils.trailingQueryPunctuationRe_, "$1")
	};
	goog.uri.utils.setParam = function (a, b, c) {
	    return goog.uri.utils.appendParam(goog.uri.utils.removeParam(a, b), b, c)
	};
	goog.uri.utils.setParamsFromMap = function (a, b) {
	    a = goog.uri.utils.splitQueryData_(a);
	    var c = a[1],
	        d = [];
	    c && c.split("&").forEach(function (e) {
	        var f = e.indexOf("=");
	        b.hasOwnProperty(0 <= f ? e.substr(0, f) : e) || d.push(e);
	    });
	    a[1] = goog.uri.utils.appendQueryData_(d.join("&"), goog.uri.utils.buildQueryDataFromMap(b));
	    return goog.uri.utils.joinQueryData_(a)
	};
	goog.uri.utils.appendPath = function (a, b) {
	    goog.uri.utils.assertNoFragmentsOrQueries_(a);
	    goog.string.endsWith(a, "/") && (a = a.substr(0, a.length - 1));
	    goog.string.startsWith(b, "/") && (b = b.substr(1));
	    return "" + a + "/" + b
	};
	goog.uri.utils.setPath = function (a, b) {
	    goog.string.startsWith(b, "/") || (b = "/" + b);
	    a = goog.uri.utils.split(a);
	    return goog.uri.utils.buildFromEncodedParts(a[goog.uri.utils.ComponentIndex.SCHEME], a[goog.uri.utils.ComponentIndex.USER_INFO], a[goog.uri.utils.ComponentIndex.DOMAIN], a[goog.uri.utils.ComponentIndex.PORT], b, a[goog.uri.utils.ComponentIndex.QUERY_DATA], a[goog.uri.utils.ComponentIndex.FRAGMENT])
	};
	goog.uri.utils.StandardQueryParam = {
	    RANDOM: "zx"
	};
	goog.uri.utils.makeUnique = function (a) {
	    return goog.uri.utils.setParam(a, goog.uri.utils.StandardQueryParam.RANDOM, goog.string.getRandomString())
	};
	var module$exports$google3$third_party$javascript$tslib$tslib = {},
	    module$contents$google3$third_party$javascript$tslib$tslib_extendStatics = Object.setPrototypeOf || {
	        __proto__: []
	    }
	instanceof Array && function (a, b) {
	    a.__proto__ = b;
	} || function (a, b) {
	    for (var c in b) b.hasOwnProperty(c) && (a[c] = b[c]);
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__extends = function (a, b) {
	    function c() {
	        this.constructor = a;
	    }
	    module$contents$google3$third_party$javascript$tslib$tslib_extendStatics(a, b);
	    a.prototype = null === b ? Object.create(b) : (c.prototype = b.prototype, new c);
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__assign = Object.assign || function (a) {
	    for (var b, c = 1, d = arguments.length; c < d; c++) {
	        b = arguments[c];
	        for (var e in b) Object.prototype.hasOwnProperty.call(b, e) && (a[e] = b[e]);
	    }
	    return a
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__rest = function (a, b) {
	    var c = {},
	        d;
	    for (d in a) Object.prototype.hasOwnProperty.call(a, d) && 0 > b.indexOf(d) && (c[d] = a[d]);
	    if (null != a && "function" === typeof Object.getOwnPropertySymbols) {
	        var e = 0;
	        for (d = Object.getOwnPropertySymbols(a); e < d.length; e++) 0 > b.indexOf(d[e]) && (c[d[e]] = a[d[e]]);
	    }
	    return c
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__decorate = function (a, b, c, d) {
	    var e = arguments.length,
	        f = 3 > e ? b : null === d ? d = Object.getOwnPropertyDescriptor(b, c) : d,
	        g;
	    if ("object" === typeof Reflect && Reflect && "function" === typeof Reflect.decorate) f = Reflect.decorate(a, b, c, d);
	    else
	        for (var h = a.length - 1; 0 <= h; h--)
	            if (g = a[h]) f = (3 > e ? g(f) : 3 < e ? g(b, c, f) : g(b, c)) || f;
	    return 3 < e && f && Object.defineProperty(b, c, f), f
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__metadata = function (a, b) {
	    if ("object" === typeof Reflect && Reflect && "function" === typeof Reflect.metadata) return Reflect.metadata(a, b)
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__param = function (a, b) {
	    return function (c, d) {
	        b(c, d, a);
	    }
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__awaiter = function (a, b, c, d) {
	    return new(c || (c = Promise))(function (e, f) {
	        function g(l) {
	            try {
	                k(d.next(l));
	            } catch (m) {
	                f(m);
	            }
	        }

	        function h(l) {
	            try {
	                k(d["throw"](l));
	            } catch (m) {
	                f(m);
	            }
	        }

	        function k(l) {
	            l.done ? e(l.value) : (new c(function (m) {
	                m(l.value);
	            })).then(g, h);
	        }
	        k((d = d.apply(a, b)).next());
	    })
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__generator = function (a, b) {
	    function c(l) {
	        return function (m) {
	            return d([l, m])
	        }
	    }

	    function d(l) {
	        if (f) throw new TypeError("Generator is already executing.");
	        for (; e;) try {
	            if (f = 1, g && (h = g[l[0] & 2 ? "return" : l[0] ? "throw" : "next"]) && !(h = h.call(g, l[1])).done) return h;
	            if (g = 0, h) l = [0, h.value];
	            switch (l[0]) {
	                case 0:
	                case 1:
	                    h = l;
	                    break;
	                case 4:
	                    return e.label++, {
	                        value: l[1],
	                        done: !1
	                    };
	                case 5:
	                    e.label++;
	                    g = l[1];
	                    l = [0];
	                    continue;
	                case 7:
	                    l = e.ops.pop();
	                    e.trys.pop();
	                    continue;
	                default:
	                    if (!(h =
	                            e.trys, h = 0 < h.length && h[h.length - 1]) && (6 === l[0] || 2 === l[0])) {
	                        e = 0;
	                        continue
	                    }
	                    if (3 === l[0] && (!h || l[1] > h[0] && l[1] < h[3])) e.label = l[1];
	                    else if (6 === l[0] && e.label < h[1]) e.label = h[1], h = l;
	                    else if (h && e.label < h[2]) e.label = h[2], e.ops.push(l);
	                    else {
	                        h[2] && e.ops.pop();
	                        e.trys.pop();
	                        continue
	                    }
	            }
	            l = b.call(a, e);
	        } catch (m) {
	            l = [6, m], g = 0;
	        } finally {
	            f = h = 0;
	        }
	        if (l[0] & 5) throw l[1];
	        return {
	            value: l[0] ? l[1] : void 0,
	            done: !0
	        }
	    }
	    var e = {
	            label: 0,
	            sent: function () {
	                if (h[0] & 1) throw h[1];
	                return h[1]
	            },
	            trys: [],
	            ops: []
	        },
	        f, g, h, k;
	    return k = {
	        next: c(0),
	        "throw": c(1),
	        "return": c(2)
	    }, "function" === typeof Symbol && (k[Symbol.iterator] = function () {
	        return k
	    }), k
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__exportStar = function (a, b) {
	    for (var c in a) b.hasOwnProperty(c) || (b[c] = a[c]);
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__values = function (a) {
	    var b = "function" === typeof Symbol && a[Symbol.iterator],
	        c = 0;
	    return b ? b.call(a) : {
	        next: function () {
	            a && c >= a.length && (a = void 0);
	            return {
	                value: a && a[c++],
	                done: !a
	            }
	        }
	    }
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__read = function (a, b) {
	    var c = "function" === typeof Symbol && a[Symbol.iterator];
	    if (!c) return a;
	    a = c.call(a);
	    var d, e = [];
	    try {
	        for (;
	            (void 0 === b || 0 < b--) && !(d = a.next()).done;) e.push(d.value);
	    } catch (g) {
	        var f = {
	            error: g
	        };
	    } finally {
	        try {
	            d && !d.done && (c = a["return"]) && c.call(a);
	        } finally {
	            if (f) throw f.error;
	        }
	    }
	    return e
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__spread = function () {
	    for (var a = [], b = 0; b < arguments.length; b++) a = a.concat(module$exports$google3$third_party$javascript$tslib$tslib.__read(arguments[b]));
	    return a
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__spreadArrays = function () {
	    for (var a = 0, b = 0, c = arguments.length; b < c; b++) a += arguments[b].length;
	    a = Array(a);
	    var d = 0;
	    for (b = 0; b < c; b++)
	        for (var e = arguments[b], f = 0, g = e.length; f < g; f++, d++) a[d] = e[f];
	    return a
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__spreadArray = function (a, b) {
	    if (!(Array.isArray(b) || b instanceof NodeList)) throw new TypeError("Expected an Array or NodeList: " + String(b));
	    for (var c = 0, d = b.length, e = a.length; c < d; c++, e++) a[e] = b[c];
	    return a
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__await = function (a) {
	    return this instanceof module$exports$google3$third_party$javascript$tslib$tslib.__await ? (this.v = a, this) : new module$exports$google3$third_party$javascript$tslib$tslib.__await(a)
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__asyncGenerator = function (a, b, c) {
	    function d(n) {
	        k[n] && (l[n] = function (r) {
	            return new Promise(function (q, p) {
	                1 < m.push([n, r, q, p]) || e(n, r);
	            })
	        });
	    }

	    function e(n, r) {
	        try {
	            var q = k[n](r);
	            q.value instanceof module$exports$google3$third_party$javascript$tslib$tslib.__await ? Promise.resolve(q.value.v).then(f, g) : h(m[0][2], q);
	        } catch (p) {
	            h(m[0][3], p);
	        }
	    }

	    function f(n) {
	        e("next", n);
	    }

	    function g(n) {
	        e("throw", n);
	    }

	    function h(n, r) {
	        (n(r), m.shift(), m.length) && e(m[0][0], m[0][1]);
	    }
	    if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
	    var k = c.apply(a, b || []),
	        l, m = [];
	    return l = {}, d("next"), d("throw"), d("return"), l[Symbol.asyncIterator] = function () {
	        return this
	    }, l
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__asyncDelegator = function (a) {
	    function b(e, f) {
	        a[e] && (c[e] = function (g) {
	            return (d = !d) ? {
	                value: new module$exports$google3$third_party$javascript$tslib$tslib.__await(a[e](g)),
	                done: "return" === e
	            } : f ? f(g) : g
	        });
	    }
	    var c, d;
	    return c = {}, b("next"), b("throw", function (e) {
	        throw e;
	    }), b("return"), c[Symbol.iterator] = function () {
	        return c
	    }, c
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__asyncValues = function (a) {
	    if (!Symbol.asyncIterator) throw new TypeError("Symbol.asyncIterator is not defined.");
	    var b = a[Symbol.asyncIterator];
	    return b ? b.call(a) : "function" === typeof __values ? __values(a) : a[Symbol.iterator]()
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__makeTemplateObject = function (a, b) {
	    Object.defineProperty ? Object.defineProperty(a, "raw", {
	        value: b
	    }) : a.raw = b;
	    return a
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__classPrivateFieldGet = function (a, b) {
	    if (!b.has(a)) throw new TypeError("attempted to get private field on non-instance");
	    return b.get(a)
	};
	module$exports$google3$third_party$javascript$tslib$tslib.__classPrivateFieldSet = function (a, b, c) {
	    if (!b.has(a)) throw new TypeError("attempted to set private field on non-instance");
	    b.set(a, c);
	    return c
	};
	var proto = {
	    tflite: {}
	};
	proto.tflite.proto = {};
	proto.tflite.proto.CoralSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.CoralSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.CoralSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.CoralSettings.toObject(a, this)
	}, proto.tflite.proto.CoralSettings.toObject = function (a, b) {
	    var c, d = {
	        device: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        performance: module$contents$jspb$Message_Message.getFieldWithDefault(b, 2, 1),
	        usbAlwaysDfu: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 3)) ? void 0 : c,
	        usbMaxBulkInQueueLength: null ==
	            (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.CoralSettings.ObjectFormat = function () {}, proto.tflite.proto.CoralSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.CoralSettings;
	    null != a.device && module$contents$jspb$Message_Message.setField(b, 1, a.device);
	    null != a.performance && module$contents$jspb$Message_Message.setField(b, 2, a.performance);
	    null != a.usbAlwaysDfu && module$contents$jspb$Message_Message.setField(b, 3, a.usbAlwaysDfu);
	    null != a.usbMaxBulkInQueueLength &&
	        module$contents$jspb$Message_Message.setField(b, 4, a.usbMaxBulkInQueueLength);
	    return b
	});
	proto.tflite.proto.CoralSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.CoralSettings.deserializeBinaryFromReader(new proto.tflite.proto.CoralSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.CoralSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readString();
	            a.setDevice(c);
	            break;
	        case 2:
	            c = b.readEnum();
	            a.setPerformance(c);
	            break;
	        case 3:
	            c = b.readBool();
	            a.setUsbAlwaysDfu(c);
	            break;
	        case 4:
	            c = b.readInt32();
	            a.setUsbMaxBulkInQueueLength(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.CoralSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.CoralSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.CoralSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeString(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeEnum(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeBool(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeInt32(4, c);
	};
	proto.tflite.proto.CoralSettings.Performance = {
	    UNDEFINED: 0,
	    MAXIMUM: 1,
	    HIGH: 2,
	    MEDIUM: 3,
	    LOW: 4
	};
	proto.tflite.proto.CoralSettings.prototype.getDevice = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 1)
	};
	proto.tflite.proto.CoralSettings.prototype.setDevice = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.CoralSettings.prototype.clearDevice = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.CoralSettings.prototype.hasDevice = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.CoralSettings.prototype.getPerformance = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 2, 1)
	};
	proto.tflite.proto.CoralSettings.prototype.setPerformance = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.proto.CoralSettings.prototype.clearPerformance = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.proto.CoralSettings.prototype.hasPerformance = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.CoralSettings.prototype.getUsbAlwaysDfu = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 3)
	};
	proto.tflite.proto.CoralSettings.prototype.setUsbAlwaysDfu = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.proto.CoralSettings.prototype.clearUsbAlwaysDfu = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.proto.CoralSettings.prototype.hasUsbAlwaysDfu = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.CoralSettings.prototype.getUsbMaxBulkInQueueLength = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 4)
	};
	proto.tflite.proto.CoralSettings.prototype.setUsbMaxBulkInQueueLength = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.proto.CoralSettings.prototype.clearUsbMaxBulkInQueueLength = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.proto.CoralSettings.prototype.hasUsbMaxBulkInQueueLength = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.CoralSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.CoralSettings, a)
	};
	proto.tflite.proto.CPUSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.CPUSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.CPUSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.CPUSettings.toObject(a, this)
	}, proto.tflite.proto.CPUSettings.toObject = function (a, b) {
	    var c = {
	        numThreads: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b, 1, -1)
	    };
	    a && (c.$jspbMessageInstance = b);
	    return c
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.CPUSettings.ObjectFormat = function () {}, proto.tflite.proto.CPUSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.CPUSettings;
	    null != a.numThreads && module$contents$jspb$Message_Message.setField(b, 1, a.numThreads);
	    return b
	});
	proto.tflite.proto.CPUSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.CPUSettings.deserializeBinaryFromReader(new proto.tflite.proto.CPUSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.CPUSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readInt32();
	            a.setNumThreads(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.CPUSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.CPUSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.CPUSettings.serializeBinaryToWriter = function (a, b) {
	    a = module$contents$jspb$Message_Message.getField(a, 1);
	    null != a && b.writeInt32(1, a);
	};
	proto.tflite.proto.CPUSettings.prototype.getNumThreads = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1, -1)
	};
	proto.tflite.proto.CPUSettings.prototype.setNumThreads = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.CPUSettings.prototype.clearNumThreads = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.CPUSettings.prototype.hasNumThreads = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.CPUSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.CPUSettings, a)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.proto.EdgeTpuDeviceSpec.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.proto.EdgeTpuDeviceSpec, module$contents$jspb$Message_Message);
	proto.tflite.proto.EdgeTpuDeviceSpec.repeatedFields_ = [3];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.EdgeTpuDeviceSpec.prototype.toObject = function (a) {
	    return proto.tflite.proto.EdgeTpuDeviceSpec.toObject(a, this)
	}, proto.tflite.proto.EdgeTpuDeviceSpec.toObject = function (a, b) {
	    var c, d = {
	        platformType: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        numChips: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        devicePathsList: null == (c = module$contents$jspb$Message_Message.getRepeatedField(b, 3)) ?
	            void 0 : c,
	        chipFamily: null == (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.EdgeTpuDeviceSpec.ObjectFormat = function () {}, proto.tflite.proto.EdgeTpuDeviceSpec.fromObject = function (a) {
	    var b = new proto.tflite.proto.EdgeTpuDeviceSpec;
	    null != a.platformType && module$contents$jspb$Message_Message.setField(b, 1, a.platformType);
	    null != a.numChips && module$contents$jspb$Message_Message.setField(b, 2, a.numChips);
	    null != a.devicePathsList && module$contents$jspb$Message_Message.setField(b, 3, a.devicePathsList);
	    null != a.chipFamily &&
	        module$contents$jspb$Message_Message.setField(b, 4, a.chipFamily);
	    return b
	});
	proto.tflite.proto.EdgeTpuDeviceSpec.deserializeBinary = function (a) {
	    return proto.tflite.proto.EdgeTpuDeviceSpec.deserializeBinaryFromReader(new proto.tflite.proto.EdgeTpuDeviceSpec, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readEnum();
	            a.setPlatformType(c);
	            break;
	        case 2:
	            c = b.readInt32();
	            a.setNumChips(c);
	            break;
	        case 3:
	            c = b.readString();
	            a.addDevicePaths(c);
	            break;
	        case 4:
	            c = b.readInt32();
	            a.setChipFamily(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.EdgeTpuDeviceSpec.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeEnum(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt32(2, c);
	    c = a.getDevicePathsList();
	    0 < c.length && b.writeRepeatedString(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeInt32(4, c);
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.PlatformType = {
	    MMIO: 0,
	    REFERENCE: 1,
	    SIMULATOR: 2,
	    REMOTE_SIMULATOR: 3
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.getPlatformType = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 1, 0)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.setPlatformType = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.clearPlatformType = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.hasPlatformType = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.getNumChips = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.setNumChips = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.clearNumChips = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.hasNumChips = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.getDevicePathsList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedField(this, 3)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.setDevicePathsList = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a || [])
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.addDevicePaths = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedField(this, 3, a, b)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.clearDevicePathsList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedField(this, 3)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.getChipFamily = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 4)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.setChipFamily = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.clearChipFamily = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.prototype.hasChipFamily = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.EdgeTpuDeviceSpec.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.EdgeTpuDeviceSpec, a)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.EdgeTpuInactivePowerConfig, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.toObject = function (a) {
	    return proto.tflite.proto.EdgeTpuInactivePowerConfig.toObject(a, this)
	}, proto.tflite.proto.EdgeTpuInactivePowerConfig.toObject = function (a, b) {
	    var c, d = {
	        inactivePowerState: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        inactiveTimeoutUs: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.EdgeTpuInactivePowerConfig.ObjectFormat = function () {}, proto.tflite.proto.EdgeTpuInactivePowerConfig.fromObject = function (a) {
	    var b = new proto.tflite.proto.EdgeTpuInactivePowerConfig;
	    null != a.inactivePowerState && module$contents$jspb$Message_Message.setField(b, 1, a.inactivePowerState);
	    null != a.inactiveTimeoutUs && module$contents$jspb$Message_Message.setField(b, 2, a.inactiveTimeoutUs);
	    return b
	});
	proto.tflite.proto.EdgeTpuInactivePowerConfig.deserializeBinary = function (a) {
	    return proto.tflite.proto.EdgeTpuInactivePowerConfig.deserializeBinaryFromReader(new proto.tflite.proto.EdgeTpuInactivePowerConfig, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readEnum();
	            a.setInactivePowerState(c);
	            break;
	        case 2:
	            c = b.readInt64();
	            a.setInactiveTimeoutUs(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.EdgeTpuInactivePowerConfig.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeEnum(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt64(2, c);
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.getInactivePowerState = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 1, 0)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.setInactivePowerState = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.clearInactivePowerState = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.hasInactivePowerState = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.getInactiveTimeoutUs = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.setInactiveTimeoutUs = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.clearInactiveTimeoutUs = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.prototype.hasInactiveTimeoutUs = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.EdgeTpuInactivePowerConfig.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.EdgeTpuInactivePowerConfig, a)
	};
	proto.tflite.proto.EdgeTpuSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.proto.EdgeTpuSettings.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.proto.EdgeTpuSettings, module$contents$jspb$Message_Message);
	proto.tflite.proto.EdgeTpuSettings.repeatedFields_ = [2];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.EdgeTpuSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.EdgeTpuSettings.toObject(a, this)
	}, proto.tflite.proto.EdgeTpuSettings.toObject = function (a, b) {
	    var c, d = {
	        inferencePowerState: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        inactivePowerConfigsList: module$contents$jspb$Message_Message.toObjectList(b.getInactivePowerConfigsList(), proto.tflite.proto.EdgeTpuInactivePowerConfig.toObject,
	            a),
	        inferencePriority: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b, 3, -1),
	        edgetpuDeviceSpec: (c = b.getEdgetpuDeviceSpec()) && proto.tflite.proto.EdgeTpuDeviceSpec.toObject(a, c),
	        modelToken: null == (c = module$contents$jspb$Message_Message.getField(b, 5)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.EdgeTpuSettings.ObjectFormat = function () {}, proto.tflite.proto.EdgeTpuSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.EdgeTpuSettings;
	    null != a.inferencePowerState && module$contents$jspb$Message_Message.setField(b, 1, a.inferencePowerState);
	    a.inactivePowerConfigsList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 2, a.inactivePowerConfigsList.map(proto.tflite.proto.EdgeTpuInactivePowerConfig.fromObject));
	    null != a.inferencePriority && module$contents$jspb$Message_Message.setField(b, 3, a.inferencePriority);
	    a.edgetpuDeviceSpec && module$contents$jspb$Message_Message.setWrapperField(b, 4, proto.tflite.proto.EdgeTpuDeviceSpec.fromObject(a.edgetpuDeviceSpec));
	    null != a.modelToken && module$contents$jspb$Message_Message.setField(b, 5, a.modelToken);
	    return b
	});
	proto.tflite.proto.EdgeTpuSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.EdgeTpuSettings.deserializeBinaryFromReader(new proto.tflite.proto.EdgeTpuSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.EdgeTpuSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readEnum();
	            a.setInferencePowerState(c);
	            break;
	        case 2:
	            c = new proto.tflite.proto.EdgeTpuInactivePowerConfig;
	            b.readMessage(c, proto.tflite.proto.EdgeTpuInactivePowerConfig.deserializeBinaryFromReader);
	            a.addInactivePowerConfigs(c);
	            break;
	        case 3:
	            c = b.readInt32();
	            a.setInferencePriority(c);
	            break;
	        case 4:
	            c = new proto.tflite.proto.EdgeTpuDeviceSpec;
	            b.readMessage(c,
	                proto.tflite.proto.EdgeTpuDeviceSpec.deserializeBinaryFromReader);
	            a.setEdgetpuDeviceSpec(c);
	            break;
	        case 5:
	            c = b.readString();
	            a.setModelToken(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.EdgeTpuSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.EdgeTpuSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeEnum(1, c);
	    c = a.getInactivePowerConfigsList();
	    0 < c.length && b.writeRepeatedMessage(2, c, proto.tflite.proto.EdgeTpuInactivePowerConfig.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeInt32(3, c);
	    c = a.getEdgetpuDeviceSpec();
	    null != c && b.writeMessage(4, c, proto.tflite.proto.EdgeTpuDeviceSpec.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a,
	        5);
	    null != c && b.writeString(5, c);
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.getInferencePowerState = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 1, 0)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.setInferencePowerState = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.clearInferencePowerState = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.hasInferencePowerState = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.getInactivePowerConfigsList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.proto.EdgeTpuInactivePowerConfig, 2)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.setInactivePowerConfigsList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 2, a)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.addInactivePowerConfigs = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 2, a, proto.tflite.proto.EdgeTpuInactivePowerConfig, b)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.clearInactivePowerConfigsList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 2)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.getInferencePriority = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 3, -1)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.setInferencePriority = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.clearInferencePriority = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.hasInferencePriority = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.getEdgetpuDeviceSpec = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.EdgeTpuDeviceSpec, 4)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.setEdgetpuDeviceSpec = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 4, a)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.clearEdgetpuDeviceSpec = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 4)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.hasEdgetpuDeviceSpec = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.getModelToken = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 5)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.setModelToken = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 5, a)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.clearModelToken = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 5)
	};
	proto.tflite.proto.EdgeTpuSettings.prototype.hasModelToken = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 5)
	};
	proto.tflite.proto.EdgeTpuSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.EdgeTpuSettings, a)
	};
	proto.tflite.proto.FallbackSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.FallbackSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.FallbackSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.FallbackSettings.toObject(a, this)
	}, proto.tflite.proto.FallbackSettings.toObject = function (a, b) {
	    var c, d = {
	        allowAutomaticFallbackOnCompilationError: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 7)) ? void 0 : c,
	        allowAutomaticFallbackOnExecutionError: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 8)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance =
	        b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.FallbackSettings.ObjectFormat = function () {}, proto.tflite.proto.FallbackSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.FallbackSettings;
	    null != a.allowAutomaticFallbackOnCompilationError && module$contents$jspb$Message_Message.setField(b, 7, a.allowAutomaticFallbackOnCompilationError);
	    null != a.allowAutomaticFallbackOnExecutionError && module$contents$jspb$Message_Message.setField(b, 8, a.allowAutomaticFallbackOnExecutionError);
	    return b
	});
	proto.tflite.proto.FallbackSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.FallbackSettings.deserializeBinaryFromReader(new proto.tflite.proto.FallbackSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.FallbackSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 7:
	            var c = b.readBool();
	            a.setAllowAutomaticFallbackOnCompilationError(c);
	            break;
	        case 8:
	            c = b.readBool();
	            a.setAllowAutomaticFallbackOnExecutionError(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.FallbackSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.FallbackSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.FallbackSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 7);
	    null != c && b.writeBool(7, c);
	    c = module$contents$jspb$Message_Message.getField(a, 8);
	    null != c && b.writeBool(8, c);
	};
	proto.tflite.proto.FallbackSettings.prototype.getAllowAutomaticFallbackOnCompilationError = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 7)
	};
	proto.tflite.proto.FallbackSettings.prototype.setAllowAutomaticFallbackOnCompilationError = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 7, a)
	};
	proto.tflite.proto.FallbackSettings.prototype.clearAllowAutomaticFallbackOnCompilationError = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 7)
	};
	proto.tflite.proto.FallbackSettings.prototype.hasAllowAutomaticFallbackOnCompilationError = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 7)
	};
	proto.tflite.proto.FallbackSettings.prototype.getAllowAutomaticFallbackOnExecutionError = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 8)
	};
	proto.tflite.proto.FallbackSettings.prototype.setAllowAutomaticFallbackOnExecutionError = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 8, a)
	};
	proto.tflite.proto.FallbackSettings.prototype.clearAllowAutomaticFallbackOnExecutionError = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 8)
	};
	proto.tflite.proto.FallbackSettings.prototype.hasAllowAutomaticFallbackOnExecutionError = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 8)
	};
	proto.tflite.proto.FallbackSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.FallbackSettings, a)
	};
	proto.tflite.proto.GPUSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.GPUSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.GPUSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.GPUSettings.toObject(a, this)
	}, proto.tflite.proto.GPUSettings.toObject = function (a, b) {
	    var c, d = {
	        isPrecisionLossAllowed: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 1)) ? void 0 : c,
	        enableQuantizedInference: module$contents$jspb$Message_Message.getBooleanFieldWithDefault(b, 2, !0),
	        forceBackend: null == (c = module$contents$jspb$Message_Message.getField(b,
	            3)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.GPUSettings.ObjectFormat = function () {}, proto.tflite.proto.GPUSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.GPUSettings;
	    null != a.isPrecisionLossAllowed && module$contents$jspb$Message_Message.setField(b, 1, a.isPrecisionLossAllowed);
	    null != a.enableQuantizedInference && module$contents$jspb$Message_Message.setField(b, 2, a.enableQuantizedInference);
	    null != a.forceBackend && module$contents$jspb$Message_Message.setField(b,
	        3, a.forceBackend);
	    return b
	});
	proto.tflite.proto.GPUSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.GPUSettings.deserializeBinaryFromReader(new proto.tflite.proto.GPUSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.GPUSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readBool();
	            a.setIsPrecisionLossAllowed(c);
	            break;
	        case 2:
	            c = b.readBool();
	            a.setEnableQuantizedInference(c);
	            break;
	        case 3:
	            c = b.readEnum();
	            a.setForceBackend(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.GPUSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.GPUSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.GPUSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeBool(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeBool(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeEnum(3, c);
	};
	proto.tflite.proto.GPUSettings.prototype.getIsPrecisionLossAllowed = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 1)
	};
	proto.tflite.proto.GPUSettings.prototype.setIsPrecisionLossAllowed = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.GPUSettings.prototype.clearIsPrecisionLossAllowed = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.GPUSettings.prototype.hasIsPrecisionLossAllowed = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.GPUSettings.prototype.getEnableQuantizedInference = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 2, !0)
	};
	proto.tflite.proto.GPUSettings.prototype.setEnableQuantizedInference = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.proto.GPUSettings.prototype.clearEnableQuantizedInference = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.proto.GPUSettings.prototype.hasEnableQuantizedInference = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.GPUSettings.prototype.getForceBackend = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 3, 0)
	};
	proto.tflite.proto.GPUSettings.prototype.setForceBackend = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.proto.GPUSettings.prototype.clearForceBackend = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.proto.GPUSettings.prototype.hasForceBackend = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.GPUSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.GPUSettings, a)
	};
	proto.tflite.proto.HexagonSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.HexagonSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.HexagonSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.HexagonSettings.toObject(a, this)
	}, proto.tflite.proto.HexagonSettings.toObject = function (a, b) {
	    var c, d = {
	        debugLevel: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        powersaveLevel: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        printGraphProfile: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 3)) ?
	            void 0 : c,
	        printGraphDebug: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 4)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.HexagonSettings.ObjectFormat = function () {}, proto.tflite.proto.HexagonSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.HexagonSettings;
	    null != a.debugLevel && module$contents$jspb$Message_Message.setField(b, 1, a.debugLevel);
	    null != a.powersaveLevel && module$contents$jspb$Message_Message.setField(b, 2, a.powersaveLevel);
	    null != a.printGraphProfile && module$contents$jspb$Message_Message.setField(b, 3, a.printGraphProfile);
	    null !=
	        a.printGraphDebug && module$contents$jspb$Message_Message.setField(b, 4, a.printGraphDebug);
	    return b
	});
	proto.tflite.proto.HexagonSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.HexagonSettings.deserializeBinaryFromReader(new proto.tflite.proto.HexagonSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.HexagonSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readInt32();
	            a.setDebugLevel(c);
	            break;
	        case 2:
	            c = b.readInt32();
	            a.setPowersaveLevel(c);
	            break;
	        case 3:
	            c = b.readBool();
	            a.setPrintGraphProfile(c);
	            break;
	        case 4:
	            c = b.readBool();
	            a.setPrintGraphDebug(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.HexagonSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.HexagonSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.HexagonSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeInt32(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt32(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeBool(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeBool(4, c);
	};
	proto.tflite.proto.HexagonSettings.prototype.getDebugLevel = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1)
	};
	proto.tflite.proto.HexagonSettings.prototype.setDebugLevel = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.HexagonSettings.prototype.clearDebugLevel = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.HexagonSettings.prototype.hasDebugLevel = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.HexagonSettings.prototype.getPowersaveLevel = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.proto.HexagonSettings.prototype.setPowersaveLevel = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.proto.HexagonSettings.prototype.clearPowersaveLevel = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.proto.HexagonSettings.prototype.hasPowersaveLevel = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.HexagonSettings.prototype.getPrintGraphProfile = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 3)
	};
	proto.tflite.proto.HexagonSettings.prototype.setPrintGraphProfile = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.proto.HexagonSettings.prototype.clearPrintGraphProfile = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.proto.HexagonSettings.prototype.hasPrintGraphProfile = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.HexagonSettings.prototype.getPrintGraphDebug = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 4)
	};
	proto.tflite.proto.HexagonSettings.prototype.setPrintGraphDebug = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.proto.HexagonSettings.prototype.clearPrintGraphDebug = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.proto.HexagonSettings.prototype.hasPrintGraphDebug = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.HexagonSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.HexagonSettings, a)
	};
	proto.tflite.proto.NNAPISettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.NNAPISettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.NNAPISettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.NNAPISettings.toObject(a, this)
	}, proto.tflite.proto.NNAPISettings.toObject = function (a, b) {
	    var c, d = {
	        acceleratorName: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        cacheDirectory: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        modelToken: null == (c = module$contents$jspb$Message_Message.getField(b, 3)) ? void 0 : c,
	        executionPreference: null ==
	            (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c,
	        noOfNnapiInstancesToCache: null == (c = module$contents$jspb$Message_Message.getField(b, 5)) ? void 0 : c,
	        fallbackSettings: (c = b.getFallbackSettings()) && proto.tflite.proto.FallbackSettings.toObject(a, c),
	        allowNnapiCpuOnAndroid10Plus: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 7)) ? void 0 : c,
	        executionPriority: null == (c = module$contents$jspb$Message_Message.getField(b, 8)) ? void 0 : c,
	        allowDynamicDimensions: null == (c = module$contents$jspb$Message_Message.getBooleanField(b,
	            9)) ? void 0 : c,
	        allowFp16PrecisionForFp32: null == (c = module$contents$jspb$Message_Message.getBooleanField(b, 10)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.NNAPISettings.ObjectFormat = function () {}, proto.tflite.proto.NNAPISettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.NNAPISettings;
	    null != a.acceleratorName && module$contents$jspb$Message_Message.setField(b, 1, a.acceleratorName);
	    null != a.cacheDirectory && module$contents$jspb$Message_Message.setField(b, 2, a.cacheDirectory);
	    null != a.modelToken && module$contents$jspb$Message_Message.setField(b, 3, a.modelToken);
	    null != a.executionPreference &&
	        module$contents$jspb$Message_Message.setField(b, 4, a.executionPreference);
	    null != a.noOfNnapiInstancesToCache && module$contents$jspb$Message_Message.setField(b, 5, a.noOfNnapiInstancesToCache);
	    a.fallbackSettings && module$contents$jspb$Message_Message.setWrapperField(b, 6, proto.tflite.proto.FallbackSettings.fromObject(a.fallbackSettings));
	    null != a.allowNnapiCpuOnAndroid10Plus && module$contents$jspb$Message_Message.setField(b, 7, a.allowNnapiCpuOnAndroid10Plus);
	    null != a.executionPriority && module$contents$jspb$Message_Message.setField(b,
	        8, a.executionPriority);
	    null != a.allowDynamicDimensions && module$contents$jspb$Message_Message.setField(b, 9, a.allowDynamicDimensions);
	    null != a.allowFp16PrecisionForFp32 && module$contents$jspb$Message_Message.setField(b, 10, a.allowFp16PrecisionForFp32);
	    return b
	});
	proto.tflite.proto.NNAPISettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.NNAPISettings.deserializeBinaryFromReader(new proto.tflite.proto.NNAPISettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.NNAPISettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readString();
	            a.setAcceleratorName(c);
	            break;
	        case 2:
	            c = b.readString();
	            a.setCacheDirectory(c);
	            break;
	        case 3:
	            c = b.readString();
	            a.setModelToken(c);
	            break;
	        case 4:
	            c = b.readEnum();
	            a.setExecutionPreference(c);
	            break;
	        case 5:
	            c = b.readInt32();
	            a.setNoOfNnapiInstancesToCache(c);
	            break;
	        case 6:
	            c = new proto.tflite.proto.FallbackSettings;
	            b.readMessage(c, proto.tflite.proto.FallbackSettings.deserializeBinaryFromReader);
	            a.setFallbackSettings(c);
	            break;
	        case 7:
	            c = b.readBool();
	            a.setAllowNnapiCpuOnAndroid10Plus(c);
	            break;
	        case 8:
	            c = b.readEnum();
	            a.setExecutionPriority(c);
	            break;
	        case 9:
	            c = b.readBool();
	            a.setAllowDynamicDimensions(c);
	            break;
	        case 10:
	            c = b.readBool();
	            a.setAllowFp16PrecisionForFp32(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.NNAPISettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.NNAPISettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.NNAPISettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeString(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeString(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeString(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeEnum(4, c);
	    c = module$contents$jspb$Message_Message.getField(a, 5);
	    null != c && b.writeInt32(5, c);
	    c = a.getFallbackSettings();
	    null != c && b.writeMessage(6, c, proto.tflite.proto.FallbackSettings.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 7);
	    null != c && b.writeBool(7, c);
	    c = module$contents$jspb$Message_Message.getField(a, 8);
	    null != c && b.writeEnum(8, c);
	    c = module$contents$jspb$Message_Message.getField(a, 9);
	    null != c && b.writeBool(9, c);
	    c = module$contents$jspb$Message_Message.getField(a, 10);
	    null != c && b.writeBool(10, c);
	};
	proto.tflite.proto.NNAPISettings.prototype.getAcceleratorName = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 1)
	};
	proto.tflite.proto.NNAPISettings.prototype.setAcceleratorName = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearAcceleratorName = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasAcceleratorName = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.NNAPISettings.prototype.getCacheDirectory = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 2)
	};
	proto.tflite.proto.NNAPISettings.prototype.setCacheDirectory = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearCacheDirectory = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasCacheDirectory = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.NNAPISettings.prototype.getModelToken = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 3)
	};
	proto.tflite.proto.NNAPISettings.prototype.setModelToken = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearModelToken = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasModelToken = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.NNAPISettings.prototype.getExecutionPreference = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 4, 0)
	};
	proto.tflite.proto.NNAPISettings.prototype.setExecutionPreference = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearExecutionPreference = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasExecutionPreference = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.NNAPISettings.prototype.getNoOfNnapiInstancesToCache = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 5)
	};
	proto.tflite.proto.NNAPISettings.prototype.setNoOfNnapiInstancesToCache = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 5, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearNoOfNnapiInstancesToCache = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 5)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasNoOfNnapiInstancesToCache = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 5)
	};
	proto.tflite.proto.NNAPISettings.prototype.getFallbackSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.FallbackSettings, 6)
	};
	proto.tflite.proto.NNAPISettings.prototype.setFallbackSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 6, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearFallbackSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 6)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasFallbackSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 6)
	};
	proto.tflite.proto.NNAPISettings.prototype.getAllowNnapiCpuOnAndroid10Plus = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 7)
	};
	proto.tflite.proto.NNAPISettings.prototype.setAllowNnapiCpuOnAndroid10Plus = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 7, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearAllowNnapiCpuOnAndroid10Plus = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 7)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasAllowNnapiCpuOnAndroid10Plus = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 7)
	};
	proto.tflite.proto.NNAPISettings.prototype.getExecutionPriority = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 8, 0)
	};
	proto.tflite.proto.NNAPISettings.prototype.setExecutionPriority = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 8, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearExecutionPriority = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 8)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasExecutionPriority = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 8)
	};
	proto.tflite.proto.NNAPISettings.prototype.getAllowDynamicDimensions = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 9)
	};
	proto.tflite.proto.NNAPISettings.prototype.setAllowDynamicDimensions = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 9, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearAllowDynamicDimensions = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 9)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasAllowDynamicDimensions = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 9)
	};
	proto.tflite.proto.NNAPISettings.prototype.getAllowFp16PrecisionForFp32 = function () {
	    return module$contents$jspb$Message_Message.getBooleanFieldWithDefault(this, 10)
	};
	proto.tflite.proto.NNAPISettings.prototype.setAllowFp16PrecisionForFp32 = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 10, a)
	};
	proto.tflite.proto.NNAPISettings.prototype.clearAllowFp16PrecisionForFp32 = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 10)
	};
	proto.tflite.proto.NNAPISettings.prototype.hasAllowFp16PrecisionForFp32 = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 10)
	};
	proto.tflite.proto.NNAPISettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.NNAPISettings, a)
	};
	proto.tflite.proto.XNNPackSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.XNNPackSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.XNNPackSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.XNNPackSettings.toObject(a, this)
	}, proto.tflite.proto.XNNPackSettings.toObject = function (a, b) {
	    var c, d = {
	        numThreads: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.XNNPackSettings.ObjectFormat = function () {}, proto.tflite.proto.XNNPackSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.XNNPackSettings;
	    null != a.numThreads && module$contents$jspb$Message_Message.setField(b, 1, a.numThreads);
	    return b
	});
	proto.tflite.proto.XNNPackSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.XNNPackSettings.deserializeBinaryFromReader(new proto.tflite.proto.XNNPackSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.XNNPackSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readInt32();
	            a.setNumThreads(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.XNNPackSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.XNNPackSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.XNNPackSettings.serializeBinaryToWriter = function (a, b) {
	    a = module$contents$jspb$Message_Message.getField(a, 1);
	    null != a && b.writeInt32(1, a);
	};
	proto.tflite.proto.XNNPackSettings.prototype.getNumThreads = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1)
	};
	proto.tflite.proto.XNNPackSettings.prototype.setNumThreads = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.XNNPackSettings.prototype.clearNumThreads = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.XNNPackSettings.prototype.hasNumThreads = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.XNNPackSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.XNNPackSettings, a)
	};
	proto.tflite.proto.TFLiteSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.TFLiteSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.TFLiteSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.TFLiteSettings.toObject(a, this)
	}, proto.tflite.proto.TFLiteSettings.toObject = function (a, b) {
	    var c, d = {
	        delegate: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        nnapiSettings: (c = b.getNnapiSettings()) && proto.tflite.proto.NNAPISettings.toObject(a, c),
	        gpuSettings: (c = b.getGpuSettings()) && proto.tflite.proto.GPUSettings.toObject(a, c),
	        hexagonSettings: (c =
	            b.getHexagonSettings()) && proto.tflite.proto.HexagonSettings.toObject(a, c),
	        xnnpackSettings: (c = b.getXnnpackSettings()) && proto.tflite.proto.XNNPackSettings.toObject(a, c),
	        cpuSettings: (c = b.getCpuSettings()) && proto.tflite.proto.CPUSettings.toObject(a, c),
	        maxDelegatedPartitions: null == (c = module$contents$jspb$Message_Message.getField(b, 7)) ? void 0 : c,
	        edgetpuSettings: (c = b.getEdgetpuSettings()) && proto.tflite.proto.EdgeTpuSettings.toObject(a, c),
	        coralSettings: (c = b.getCoralSettings()) && proto.tflite.proto.CoralSettings.toObject(a,
	            c),
	        fallbackSettings: (c = b.getFallbackSettings()) && proto.tflite.proto.FallbackSettings.toObject(a, c)
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.TFLiteSettings.ObjectFormat = function () {}, proto.tflite.proto.TFLiteSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.TFLiteSettings;
	    null != a.delegate && module$contents$jspb$Message_Message.setField(b, 1, a.delegate);
	    a.nnapiSettings && module$contents$jspb$Message_Message.setWrapperField(b, 2, proto.tflite.proto.NNAPISettings.fromObject(a.nnapiSettings));
	    a.gpuSettings && module$contents$jspb$Message_Message.setWrapperField(b,
	        3, proto.tflite.proto.GPUSettings.fromObject(a.gpuSettings));
	    a.hexagonSettings && module$contents$jspb$Message_Message.setWrapperField(b, 4, proto.tflite.proto.HexagonSettings.fromObject(a.hexagonSettings));
	    a.xnnpackSettings && module$contents$jspb$Message_Message.setWrapperField(b, 5, proto.tflite.proto.XNNPackSettings.fromObject(a.xnnpackSettings));
	    a.cpuSettings && module$contents$jspb$Message_Message.setWrapperField(b, 6, proto.tflite.proto.CPUSettings.fromObject(a.cpuSettings));
	    null != a.maxDelegatedPartitions &&
	        module$contents$jspb$Message_Message.setField(b, 7, a.maxDelegatedPartitions);
	    a.edgetpuSettings && module$contents$jspb$Message_Message.setWrapperField(b, 8, proto.tflite.proto.EdgeTpuSettings.fromObject(a.edgetpuSettings));
	    a.coralSettings && module$contents$jspb$Message_Message.setWrapperField(b, 10, proto.tflite.proto.CoralSettings.fromObject(a.coralSettings));
	    a.fallbackSettings && module$contents$jspb$Message_Message.setWrapperField(b, 9, proto.tflite.proto.FallbackSettings.fromObject(a.fallbackSettings));
	    return b
	});
	proto.tflite.proto.TFLiteSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.TFLiteSettings.deserializeBinaryFromReader(new proto.tflite.proto.TFLiteSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.TFLiteSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readEnum();
	            a.setDelegate(c);
	            break;
	        case 2:
	            c = new proto.tflite.proto.NNAPISettings;
	            b.readMessage(c, proto.tflite.proto.NNAPISettings.deserializeBinaryFromReader);
	            a.setNnapiSettings(c);
	            break;
	        case 3:
	            c = new proto.tflite.proto.GPUSettings;
	            b.readMessage(c, proto.tflite.proto.GPUSettings.deserializeBinaryFromReader);
	            a.setGpuSettings(c);
	            break;
	        case 4:
	            c = new proto.tflite.proto.HexagonSettings;
	            b.readMessage(c, proto.tflite.proto.HexagonSettings.deserializeBinaryFromReader);
	            a.setHexagonSettings(c);
	            break;
	        case 5:
	            c = new proto.tflite.proto.XNNPackSettings;
	            b.readMessage(c, proto.tflite.proto.XNNPackSettings.deserializeBinaryFromReader);
	            a.setXnnpackSettings(c);
	            break;
	        case 6:
	            c = new proto.tflite.proto.CPUSettings;
	            b.readMessage(c, proto.tflite.proto.CPUSettings.deserializeBinaryFromReader);
	            a.setCpuSettings(c);
	            break;
	        case 7:
	            c = b.readInt32();
	            a.setMaxDelegatedPartitions(c);
	            break;
	        case 8:
	            c = new proto.tflite.proto.EdgeTpuSettings;
	            b.readMessage(c, proto.tflite.proto.EdgeTpuSettings.deserializeBinaryFromReader);
	            a.setEdgetpuSettings(c);
	            break;
	        case 10:
	            c = new proto.tflite.proto.CoralSettings;
	            b.readMessage(c, proto.tflite.proto.CoralSettings.deserializeBinaryFromReader);
	            a.setCoralSettings(c);
	            break;
	        case 9:
	            c = new proto.tflite.proto.FallbackSettings;
	            b.readMessage(c, proto.tflite.proto.FallbackSettings.deserializeBinaryFromReader);
	            a.setFallbackSettings(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.TFLiteSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.TFLiteSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.TFLiteSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeEnum(1, c);
	    c = a.getNnapiSettings();
	    null != c && b.writeMessage(2, c, proto.tflite.proto.NNAPISettings.serializeBinaryToWriter);
	    c = a.getGpuSettings();
	    null != c && b.writeMessage(3, c, proto.tflite.proto.GPUSettings.serializeBinaryToWriter);
	    c = a.getHexagonSettings();
	    null != c && b.writeMessage(4, c, proto.tflite.proto.HexagonSettings.serializeBinaryToWriter);
	    c = a.getXnnpackSettings();
	    null != c && b.writeMessage(5, c, proto.tflite.proto.XNNPackSettings.serializeBinaryToWriter);
	    c = a.getCpuSettings();
	    null != c && b.writeMessage(6, c, proto.tflite.proto.CPUSettings.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 7);
	    null != c && b.writeInt32(7, c);
	    c = a.getEdgetpuSettings();
	    null != c && b.writeMessage(8, c, proto.tflite.proto.EdgeTpuSettings.serializeBinaryToWriter);
	    c = a.getCoralSettings();
	    null != c && b.writeMessage(10, c, proto.tflite.proto.CoralSettings.serializeBinaryToWriter);
	    c = a.getFallbackSettings();
	    null != c && b.writeMessage(9, c, proto.tflite.proto.FallbackSettings.serializeBinaryToWriter);
	};
	proto.tflite.proto.TFLiteSettings.prototype.getDelegate = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 1, 0)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setDelegate = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearDelegate = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasDelegate = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getNnapiSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.NNAPISettings, 2)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setNnapiSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 2, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearNnapiSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 2)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasNnapiSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getGpuSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.GPUSettings, 3)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setGpuSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 3, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearGpuSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 3)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasGpuSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getHexagonSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.HexagonSettings, 4)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setHexagonSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 4, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearHexagonSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 4)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasHexagonSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getXnnpackSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.XNNPackSettings, 5)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setXnnpackSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 5, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearXnnpackSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 5)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasXnnpackSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 5)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getCpuSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.CPUSettings, 6)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setCpuSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 6, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearCpuSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 6)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasCpuSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 6)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getMaxDelegatedPartitions = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 7)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setMaxDelegatedPartitions = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 7, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearMaxDelegatedPartitions = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 7)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasMaxDelegatedPartitions = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 7)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getEdgetpuSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.EdgeTpuSettings, 8)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setEdgetpuSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 8, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearEdgetpuSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 8)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasEdgetpuSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 8)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getCoralSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.CoralSettings, 10)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setCoralSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 10, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearCoralSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 10)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasCoralSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 10)
	};
	proto.tflite.proto.TFLiteSettings.prototype.getFallbackSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.FallbackSettings, 9)
	};
	proto.tflite.proto.TFLiteSettings.prototype.setFallbackSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 9, a)
	};
	proto.tflite.proto.TFLiteSettings.prototype.clearFallbackSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 9)
	};
	proto.tflite.proto.TFLiteSettings.prototype.hasFallbackSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 9)
	};
	proto.tflite.proto.TFLiteSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.TFLiteSettings, a)
	};
	proto.tflite.proto.ComputeSettings = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.proto.ComputeSettings, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.proto.ComputeSettings.prototype.toObject = function (a) {
	    return proto.tflite.proto.ComputeSettings.toObject(a, this)
	}, proto.tflite.proto.ComputeSettings.toObject = function (a, b) {
	    var c, d = {
	        preference: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        tfliteSettings: (c = b.getTfliteSettings()) && proto.tflite.proto.TFLiteSettings.toObject(a, c),
	        modelNamespaceForStatistics: null == (c = module$contents$jspb$Message_Message.getField(b,
	            3)) ? void 0 : c,
	        modelIdentifierForStatistics: null == (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.proto.ComputeSettings.ObjectFormat = function () {}, proto.tflite.proto.ComputeSettings.fromObject = function (a) {
	    var b = new proto.tflite.proto.ComputeSettings;
	    null != a.preference && module$contents$jspb$Message_Message.setField(b, 1, a.preference);
	    a.tfliteSettings && module$contents$jspb$Message_Message.setWrapperField(b, 2, proto.tflite.proto.TFLiteSettings.fromObject(a.tfliteSettings));
	    null != a.modelNamespaceForStatistics && module$contents$jspb$Message_Message.setField(b,
	        3, a.modelNamespaceForStatistics);
	    null != a.modelIdentifierForStatistics && module$contents$jspb$Message_Message.setField(b, 4, a.modelIdentifierForStatistics);
	    return b
	});
	proto.tflite.proto.ComputeSettings.deserializeBinary = function (a) {
	    return proto.tflite.proto.ComputeSettings.deserializeBinaryFromReader(new proto.tflite.proto.ComputeSettings, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.proto.ComputeSettings.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readEnum();
	            a.setPreference(c);
	            break;
	        case 2:
	            c = new proto.tflite.proto.TFLiteSettings;
	            b.readMessage(c, proto.tflite.proto.TFLiteSettings.deserializeBinaryFromReader);
	            a.setTfliteSettings(c);
	            break;
	        case 3:
	            c = b.readString();
	            a.setModelNamespaceForStatistics(c);
	            break;
	        case 4:
	            c = b.readString();
	            a.setModelIdentifierForStatistics(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.proto.ComputeSettings.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.proto.ComputeSettings.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.proto.ComputeSettings.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeEnum(1, c);
	    c = a.getTfliteSettings();
	    null != c && b.writeMessage(2, c, proto.tflite.proto.TFLiteSettings.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeString(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeString(4, c);
	};
	proto.tflite.proto.ComputeSettings.prototype.getPreference = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 1, 0)
	};
	proto.tflite.proto.ComputeSettings.prototype.setPreference = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.proto.ComputeSettings.prototype.clearPreference = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.proto.ComputeSettings.prototype.hasPreference = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.proto.ComputeSettings.prototype.getTfliteSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.TFLiteSettings, 2)
	};
	proto.tflite.proto.ComputeSettings.prototype.setTfliteSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 2, a)
	};
	proto.tflite.proto.ComputeSettings.prototype.clearTfliteSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 2)
	};
	proto.tflite.proto.ComputeSettings.prototype.hasTfliteSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.proto.ComputeSettings.prototype.getModelNamespaceForStatistics = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 3)
	};
	proto.tflite.proto.ComputeSettings.prototype.setModelNamespaceForStatistics = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.proto.ComputeSettings.prototype.clearModelNamespaceForStatistics = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.proto.ComputeSettings.prototype.hasModelNamespaceForStatistics = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.proto.ComputeSettings.prototype.getModelIdentifierForStatistics = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 4)
	};
	proto.tflite.proto.ComputeSettings.prototype.setModelIdentifierForStatistics = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.proto.ComputeSettings.prototype.clearModelIdentifierForStatistics = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.proto.ComputeSettings.prototype.hasModelIdentifierForStatistics = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.proto.ComputeSettings.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.proto.ComputeSettings, a)
	};
	proto.tflite.task = {};
	proto.tflite.task.core = {};
	proto.tflite.task.core.FileDescriptorMeta = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.task.core.FileDescriptorMeta, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.core.FileDescriptorMeta.prototype.toObject = function (a) {
	    return proto.tflite.task.core.FileDescriptorMeta.toObject(a, this)
	}, proto.tflite.task.core.FileDescriptorMeta.toObject = function (a, b) {
	    var c, d = {
	        fd: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        length: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        offset: null == (c = module$contents$jspb$Message_Message.getField(b, 3)) ? void 0 : c
	    };
	    a &&
	        (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.core.FileDescriptorMeta.ObjectFormat = function () {}, proto.tflite.task.core.FileDescriptorMeta.fromObject = function (a) {
	    var b = new proto.tflite.task.core.FileDescriptorMeta;
	    null != a.fd && module$contents$jspb$Message_Message.setField(b, 1, a.fd);
	    null != a.length && module$contents$jspb$Message_Message.setField(b, 2, a.length);
	    null != a.offset && module$contents$jspb$Message_Message.setField(b, 3, a.offset);
	    return b
	});
	proto.tflite.task.core.FileDescriptorMeta.deserializeBinary = function (a) {
	    return proto.tflite.task.core.FileDescriptorMeta.deserializeBinaryFromReader(new proto.tflite.task.core.FileDescriptorMeta, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.core.FileDescriptorMeta.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readInt32();
	            a.setFd(c);
	            break;
	        case 2:
	            c = b.readInt64();
	            a.setLength(c);
	            break;
	        case 3:
	            c = b.readInt64();
	            a.setOffset(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.core.FileDescriptorMeta.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.core.FileDescriptorMeta.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeInt32(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt64(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeInt64(3, c);
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.getFd = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.setFd = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.clearFd = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.hasFd = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.getLength = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.setLength = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.clearLength = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.hasLength = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.getOffset = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 3)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.setOffset = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.clearOffset = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.core.FileDescriptorMeta.prototype.hasOffset = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.core.FileDescriptorMeta.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.core.FileDescriptorMeta, a)
	};
	proto.tflite.task.core.ExternalFile = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.task.core.ExternalFile, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.core.ExternalFile.prototype.toObject = function (a) {
	    return proto.tflite.task.core.ExternalFile.toObject(a, this)
	}, proto.tflite.task.core.ExternalFile.toObject = function (a, b) {
	    var c, d = {
	        fileName: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        fileContent: b.getFileContent_asB64(),
	        fileDescriptorMeta: (c = b.getFileDescriptorMeta()) && proto.tflite.task.core.FileDescriptorMeta.toObject(a, c)
	    };
	    a && (d.$jspbMessageInstance =
	        b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.core.ExternalFile.ObjectFormat = function () {}, proto.tflite.task.core.ExternalFile.fromObject = function (a) {
	    var b = new proto.tflite.task.core.ExternalFile;
	    null != a.fileName && module$contents$jspb$Message_Message.setField(b, 1, a.fileName);
	    null != a.fileContent && module$contents$jspb$Message_Message.setField(b, 2, a.fileContent);
	    a.fileDescriptorMeta && module$contents$jspb$Message_Message.setWrapperField(b, 4, proto.tflite.task.core.FileDescriptorMeta.fromObject(a.fileDescriptorMeta));
	    return b
	});
	proto.tflite.task.core.ExternalFile.deserializeBinary = function (a) {
	    return proto.tflite.task.core.ExternalFile.deserializeBinaryFromReader(new proto.tflite.task.core.ExternalFile, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.core.ExternalFile.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readString();
	            a.setFileName(c);
	            break;
	        case 2:
	            c = b.readBytes();
	            a.setFileContent(c);
	            break;
	        case 4:
	            c = new proto.tflite.task.core.FileDescriptorMeta;
	            b.readMessage(c, proto.tflite.task.core.FileDescriptorMeta.deserializeBinaryFromReader);
	            a.setFileDescriptorMeta(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.core.ExternalFile.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.core.ExternalFile.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.core.ExternalFile.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeString(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeBytes(2, c);
	    c = a.getFileDescriptorMeta();
	    null != c && b.writeMessage(4, c, proto.tflite.task.core.FileDescriptorMeta.serializeBinaryToWriter);
	};
	proto.tflite.task.core.ExternalFile.prototype.getFileName = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 1)
	};
	proto.tflite.task.core.ExternalFile.prototype.setFileName = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.task.core.ExternalFile.prototype.clearFileName = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.task.core.ExternalFile.prototype.hasFileName = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.core.ExternalFile.prototype.getFileContent = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 2)
	};
	proto.tflite.task.core.ExternalFile.prototype.getFileContent_asB64 = function () {
	    return module$contents$jspb$Message_Message.bytesAsB64(this.getFileContent())
	};
	proto.tflite.task.core.ExternalFile.prototype.getFileContent_asU8 = function () {
	    return module$contents$jspb$Message_Message.bytesAsU8(this.getFileContent())
	};
	proto.tflite.task.core.ExternalFile.prototype.setFileContent = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.core.ExternalFile.prototype.clearFileContent = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.core.ExternalFile.prototype.hasFileContent = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.core.ExternalFile.prototype.getFileDescriptorMeta = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.task.core.FileDescriptorMeta, 4)
	};
	proto.tflite.task.core.ExternalFile.prototype.setFileDescriptorMeta = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 4, a)
	};
	proto.tflite.task.core.ExternalFile.prototype.clearFileDescriptorMeta = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 4)
	};
	proto.tflite.task.core.ExternalFile.prototype.hasFileDescriptorMeta = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.core.ExternalFile.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.core.ExternalFile, a)
	};
	proto.tflite.task.vision = {};
	proto.tflite.task.vision.BoundingBox = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.task.vision.BoundingBox, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.BoundingBox.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.BoundingBox.toObject(a, this)
	}, proto.tflite.task.vision.BoundingBox.toObject = function (a, b) {
	    var c, d = {
	        originX: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        originY: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        width: null == (c = module$contents$jspb$Message_Message.getField(b, 3)) ? void 0 : c,
	        height: null ==
	            (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.BoundingBox.ObjectFormat = function () {}, proto.tflite.task.vision.BoundingBox.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.BoundingBox;
	    null != a.originX && module$contents$jspb$Message_Message.setField(b, 1, a.originX);
	    null != a.originY && module$contents$jspb$Message_Message.setField(b, 2, a.originY);
	    null != a.width && module$contents$jspb$Message_Message.setField(b, 3, a.width);
	    null != a.height && module$contents$jspb$Message_Message.setField(b,
	        4, a.height);
	    return b
	});
	proto.tflite.task.vision.BoundingBox.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.BoundingBox.deserializeBinaryFromReader(new proto.tflite.task.vision.BoundingBox, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.BoundingBox.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readInt32();
	            a.setOriginX(c);
	            break;
	        case 2:
	            c = b.readInt32();
	            a.setOriginY(c);
	            break;
	        case 3:
	            c = b.readInt32();
	            a.setWidth(c);
	            break;
	        case 4:
	            c = b.readInt32();
	            a.setHeight(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.BoundingBox.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.BoundingBox.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.BoundingBox.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeInt32(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt32(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeInt32(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeInt32(4, c);
	};
	proto.tflite.task.vision.BoundingBox.prototype.getOriginX = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1)
	};
	proto.tflite.task.vision.BoundingBox.prototype.setOriginX = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.task.vision.BoundingBox.prototype.clearOriginX = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.task.vision.BoundingBox.prototype.hasOriginX = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.vision.BoundingBox.prototype.getOriginY = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.task.vision.BoundingBox.prototype.setOriginY = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.BoundingBox.prototype.clearOriginY = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.BoundingBox.prototype.hasOriginY = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.BoundingBox.prototype.getWidth = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 3)
	};
	proto.tflite.task.vision.BoundingBox.prototype.setWidth = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.BoundingBox.prototype.clearWidth = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.BoundingBox.prototype.hasWidth = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.BoundingBox.prototype.getHeight = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 4)
	};
	proto.tflite.task.vision.BoundingBox.prototype.setHeight = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.task.vision.BoundingBox.prototype.clearHeight = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.task.vision.BoundingBox.prototype.hasHeight = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.vision.BoundingBox.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.BoundingBox, a)
	};
	proto.tflite.task.vision.Class = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.task.vision.Class, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Class.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Class.toObject(a, this)
	}, proto.tflite.task.vision.Class.toObject = function (a, b) {
	    var c, d = {
	        index: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        score: null == (c = module$contents$jspb$Message_Message.getOptionalFloatingPointField(b, 2)) ? void 0 : c,
	        displayName: null == (c = module$contents$jspb$Message_Message.getField(b, 3)) ? void 0 : c,
	        className: null ==
	            (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Class.ObjectFormat = function () {}, proto.tflite.task.vision.Class.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Class;
	    null != a.index && module$contents$jspb$Message_Message.setField(b, 1, a.index);
	    null != a.score && module$contents$jspb$Message_Message.setField(b, 2, a.score);
	    null != a.displayName && module$contents$jspb$Message_Message.setField(b, 3, a.displayName);
	    null != a.className && module$contents$jspb$Message_Message.setField(b,
	        4, a.className);
	    return b
	});
	proto.tflite.task.vision.Class.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Class.deserializeBinaryFromReader(new proto.tflite.task.vision.Class, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Class.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readInt32();
	            a.setIndex(c);
	            break;
	        case 2:
	            c = b.readFloat();
	            a.setScore(c);
	            break;
	        case 3:
	            c = b.readString();
	            a.setDisplayName(c);
	            break;
	        case 4:
	            c = b.readString();
	            a.setClassName(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Class.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Class.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Class.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeInt32(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeFloat(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeString(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeString(4, c);
	};
	proto.tflite.task.vision.Class.prototype.getIndex = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1)
	};
	proto.tflite.task.vision.Class.prototype.setIndex = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.task.vision.Class.prototype.clearIndex = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.task.vision.Class.prototype.hasIndex = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.vision.Class.prototype.getScore = function () {
	    return module$contents$jspb$Message_Message.getFloatingPointFieldWithDefault(this, 2)
	};
	proto.tflite.task.vision.Class.prototype.setScore = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.Class.prototype.clearScore = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.Class.prototype.hasScore = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.Class.prototype.getDisplayName = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 3)
	};
	proto.tflite.task.vision.Class.prototype.setDisplayName = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.Class.prototype.clearDisplayName = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.Class.prototype.hasDisplayName = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.Class.prototype.getClassName = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 4)
	};
	proto.tflite.task.vision.Class.prototype.setClassName = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.task.vision.Class.prototype.clearClassName = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.task.vision.Class.prototype.hasClassName = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.vision.Class.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Class, a)
	};
	proto.tflite.task.vision.Classifications = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.Classifications.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.Classifications, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.Classifications.repeatedFields_ = [1];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Classifications.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Classifications.toObject(a, this)
	}, proto.tflite.task.vision.Classifications.toObject = function (a, b) {
	    var c, d = {
	        classesList: module$contents$jspb$Message_Message.toObjectList(b.getClassesList(), proto.tflite.task.vision.Class.toObject, a),
	        headIndex: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance =
	        b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Classifications.ObjectFormat = function () {}, proto.tflite.task.vision.Classifications.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Classifications;
	    a.classesList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 1, a.classesList.map(proto.tflite.task.vision.Class.fromObject));
	    null != a.headIndex && module$contents$jspb$Message_Message.setField(b, 2, a.headIndex);
	    return b
	});
	proto.tflite.task.vision.Classifications.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Classifications.deserializeBinaryFromReader(new proto.tflite.task.vision.Classifications, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Classifications.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = new proto.tflite.task.vision.Class;
	            b.readMessage(c, proto.tflite.task.vision.Class.deserializeBinaryFromReader);
	            a.addClasses(c);
	            break;
	        case 2:
	            c = b.readInt32();
	            a.setHeadIndex(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Classifications.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Classifications.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Classifications.serializeBinaryToWriter = function (a, b) {
	    var c = a.getClassesList();
	    0 < c.length && b.writeRepeatedMessage(1, c, proto.tflite.task.vision.Class.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt32(2, c);
	};
	proto.tflite.task.vision.Classifications.prototype.getClassesList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Class, 1)
	};
	proto.tflite.task.vision.Classifications.prototype.setClassesList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 1, a)
	};
	proto.tflite.task.vision.Classifications.prototype.addClasses = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 1, a, proto.tflite.task.vision.Class, b)
	};
	proto.tflite.task.vision.Classifications.prototype.clearClassesList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 1)
	};
	proto.tflite.task.vision.Classifications.prototype.getHeadIndex = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.task.vision.Classifications.prototype.setHeadIndex = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.Classifications.prototype.clearHeadIndex = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.Classifications.prototype.hasHeadIndex = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.Classifications.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Classifications, a)
	};
	proto.tflite.task.vision.ClassificationResult = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.ClassificationResult.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.ClassificationResult, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.ClassificationResult.repeatedFields_ = [1];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.ClassificationResult.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.ClassificationResult.toObject(a, this)
	}, proto.tflite.task.vision.ClassificationResult.toObject = function (a, b) {
	    var c = {
	        classificationsList: module$contents$jspb$Message_Message.toObjectList(b.getClassificationsList(), proto.tflite.task.vision.Classifications.toObject, a)
	    };
	    a && (c.$jspbMessageInstance = b);
	    return c
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.ClassificationResult.ObjectFormat = function () {}, proto.tflite.task.vision.ClassificationResult.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.ClassificationResult;
	    a.classificationsList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 1, a.classificationsList.map(proto.tflite.task.vision.Classifications.fromObject));
	    return b
	});
	proto.tflite.task.vision.ClassificationResult.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.ClassificationResult.deserializeBinaryFromReader(new proto.tflite.task.vision.ClassificationResult, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.ClassificationResult.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = new proto.tflite.task.vision.Classifications;
	            b.readMessage(c, proto.tflite.task.vision.Classifications.deserializeBinaryFromReader);
	            a.addClassifications(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.ClassificationResult.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.ClassificationResult.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.ClassificationResult.serializeBinaryToWriter = function (a, b) {
	    a = a.getClassificationsList();
	    0 < a.length && b.writeRepeatedMessage(1, a, proto.tflite.task.vision.Classifications.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.ClassificationResult.prototype.getClassificationsList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Classifications, 1)
	};
	proto.tflite.task.vision.ClassificationResult.prototype.setClassificationsList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 1, a)
	};
	proto.tflite.task.vision.ClassificationResult.prototype.addClassifications = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 1, a, proto.tflite.task.vision.Classifications, b)
	};
	proto.tflite.task.vision.ClassificationResult.prototype.clearClassificationsList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 1)
	};
	proto.tflite.task.vision.ClassificationResult.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.ClassificationResult, a)
	};
	proto.tflite.task.vision.Detection = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.Detection.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.Detection, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.Detection.repeatedFields_ = [3];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Detection.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Detection.toObject(a, this)
	}, proto.tflite.task.vision.Detection.toObject = function (a, b) {
	    var c, d = {
	        boundingBox: (c = b.getBoundingBox()) && proto.tflite.task.vision.BoundingBox.toObject(a, c),
	        classesList: module$contents$jspb$Message_Message.toObjectList(b.getClassesList(), proto.tflite.task.vision.Class.toObject, a)
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Detection.ObjectFormat = function () {}, proto.tflite.task.vision.Detection.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Detection;
	    a.boundingBox && module$contents$jspb$Message_Message.setWrapperField(b, 2, proto.tflite.task.vision.BoundingBox.fromObject(a.boundingBox));
	    a.classesList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 3, a.classesList.map(proto.tflite.task.vision.Class.fromObject));
	    return b
	});
	proto.tflite.task.vision.Detection.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Detection.deserializeBinaryFromReader(new proto.tflite.task.vision.Detection, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Detection.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 2:
	            var c = new proto.tflite.task.vision.BoundingBox;
	            b.readMessage(c, proto.tflite.task.vision.BoundingBox.deserializeBinaryFromReader);
	            a.setBoundingBox(c);
	            break;
	        case 3:
	            c = new proto.tflite.task.vision.Class;
	            b.readMessage(c, proto.tflite.task.vision.Class.deserializeBinaryFromReader);
	            a.addClasses(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Detection.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Detection.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Detection.serializeBinaryToWriter = function (a, b) {
	    var c = a.getBoundingBox();
	    null != c && b.writeMessage(2, c, proto.tflite.task.vision.BoundingBox.serializeBinaryToWriter);
	    c = a.getClassesList();
	    0 < c.length && b.writeRepeatedMessage(3, c, proto.tflite.task.vision.Class.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.Detection.prototype.getBoundingBox = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.task.vision.BoundingBox, 2)
	};
	proto.tflite.task.vision.Detection.prototype.setBoundingBox = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 2, a)
	};
	proto.tflite.task.vision.Detection.prototype.clearBoundingBox = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 2)
	};
	proto.tflite.task.vision.Detection.prototype.hasBoundingBox = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.Detection.prototype.getClassesList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Class, 3)
	};
	proto.tflite.task.vision.Detection.prototype.setClassesList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 3, a)
	};
	proto.tflite.task.vision.Detection.prototype.addClasses = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 3, a, proto.tflite.task.vision.Class, b)
	};
	proto.tflite.task.vision.Detection.prototype.clearClassesList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 3)
	};
	proto.tflite.task.vision.Detection.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Detection, a)
	};
	proto.tflite.task.vision.DetectionResult = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.DetectionResult.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.DetectionResult, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.DetectionResult.repeatedFields_ = [1];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.DetectionResult.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.DetectionResult.toObject(a, this)
	}, proto.tflite.task.vision.DetectionResult.toObject = function (a, b) {
	    var c = {
	        detectionsList: module$contents$jspb$Message_Message.toObjectList(b.getDetectionsList(), proto.tflite.task.vision.Detection.toObject, a)
	    };
	    a && (c.$jspbMessageInstance = b);
	    return c
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.DetectionResult.ObjectFormat = function () {}, proto.tflite.task.vision.DetectionResult.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.DetectionResult;
	    a.detectionsList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 1, a.detectionsList.map(proto.tflite.task.vision.Detection.fromObject));
	    return b
	});
	proto.tflite.task.vision.DetectionResult.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.DetectionResult.deserializeBinaryFromReader(new proto.tflite.task.vision.DetectionResult, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.DetectionResult.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = new proto.tflite.task.vision.Detection;
	            b.readMessage(c, proto.tflite.task.vision.Detection.deserializeBinaryFromReader);
	            a.addDetections(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.DetectionResult.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.DetectionResult.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.DetectionResult.serializeBinaryToWriter = function (a, b) {
	    a = a.getDetectionsList();
	    0 < a.length && b.writeRepeatedMessage(1, a, proto.tflite.task.vision.Detection.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.DetectionResult.prototype.getDetectionsList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Detection, 1)
	};
	proto.tflite.task.vision.DetectionResult.prototype.setDetectionsList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 1, a)
	};
	proto.tflite.task.vision.DetectionResult.prototype.addDetections = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 1, a, proto.tflite.task.vision.Detection, b)
	};
	proto.tflite.task.vision.DetectionResult.prototype.clearDetectionsList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 1)
	};
	proto.tflite.task.vision.DetectionResult.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.DetectionResult, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.ImageClassifierOptions.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.ImageClassifierOptions, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.ImageClassifierOptions.repeatedFields_ = [4, 5];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.ImageClassifierOptions.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.ImageClassifierOptions.toObject(a, this)
	}, proto.tflite.task.vision.ImageClassifierOptions.toObject = function (a, b) {
	    var c, d = {
	        modelFileWithMetadata: (c = b.getModelFileWithMetadata()) && proto.tflite.task.core.ExternalFile.toObject(a, c),
	        displayNamesLocale: module$contents$jspb$Message_Message.getStringFieldWithDefault(b, 11, "en"),
	        maxResults: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b,
	            2, -1),
	        scoreThreshold: null == (c = module$contents$jspb$Message_Message.getOptionalFloatingPointField(b, 3)) ? void 0 : c,
	        classNameWhitelistList: null == (c = module$contents$jspb$Message_Message.getRepeatedField(b, 4)) ? void 0 : c,
	        classNameBlacklistList: null == (c = module$contents$jspb$Message_Message.getRepeatedField(b, 5)) ? void 0 : c,
	        numThreads: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b, 13, -1),
	        computeSettings: (c = b.getComputeSettings()) && proto.tflite.proto.ComputeSettings.toObject(a, c)
	    };
	    a && (d.$jspbMessageInstance =
	        b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.ImageClassifierOptions.ObjectFormat = function () {}, proto.tflite.task.vision.ImageClassifierOptions.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.ImageClassifierOptions;
	    a.modelFileWithMetadata && module$contents$jspb$Message_Message.setWrapperField(b, 10, proto.tflite.task.core.ExternalFile.fromObject(a.modelFileWithMetadata));
	    null != a.displayNamesLocale && module$contents$jspb$Message_Message.setField(b, 11, a.displayNamesLocale);
	    null != a.maxResults && module$contents$jspb$Message_Message.setField(b, 2, a.maxResults);
	    null != a.scoreThreshold && module$contents$jspb$Message_Message.setField(b, 3, a.scoreThreshold);
	    null != a.classNameWhitelistList && module$contents$jspb$Message_Message.setField(b, 4, a.classNameWhitelistList);
	    null != a.classNameBlacklistList && module$contents$jspb$Message_Message.setField(b, 5, a.classNameBlacklistList);
	    null != a.numThreads && module$contents$jspb$Message_Message.setField(b, 13, a.numThreads);
	    a.computeSettings && module$contents$jspb$Message_Message.setWrapperField(b,
	        9, proto.tflite.proto.ComputeSettings.fromObject(a.computeSettings));
	    return b
	});
	proto.tflite.task.vision.ImageClassifierOptions.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.ImageClassifierOptions.deserializeBinaryFromReader(new proto.tflite.task.vision.ImageClassifierOptions, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.ImageClassifierOptions.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 10:
	            var c = new proto.tflite.task.core.ExternalFile;
	            b.readMessage(c, proto.tflite.task.core.ExternalFile.deserializeBinaryFromReader);
	            a.setModelFileWithMetadata(c);
	            break;
	        case 11:
	            c = b.readString();
	            a.setDisplayNamesLocale(c);
	            break;
	        case 2:
	            c = b.readInt32();
	            a.setMaxResults(c);
	            break;
	        case 3:
	            c = b.readFloat();
	            a.setScoreThreshold(c);
	            break;
	        case 4:
	            c = b.readString();
	            a.addClassNameWhitelist(c);
	            break;
	        case 5:
	            c = b.readString();
	            a.addClassNameBlacklist(c);
	            break;
	        case 13:
	            c = b.readInt32();
	            a.setNumThreads(c);
	            break;
	        case 9:
	            c = new proto.tflite.proto.ComputeSettings;
	            b.readMessage(c, proto.tflite.proto.ComputeSettings.deserializeBinaryFromReader);
	            a.setComputeSettings(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.ImageClassifierOptions.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.ImageClassifierOptions.serializeBinaryToWriter = function (a, b) {
	    var c = a.getModelFileWithMetadata();
	    null != c && b.writeMessage(10, c, proto.tflite.task.core.ExternalFile.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 11);
	    null != c && b.writeString(11, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt32(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeFloat(3, c);
	    c = a.getClassNameWhitelistList();
	    0 < c.length && b.writeRepeatedString(4,
	        c);
	    c = a.getClassNameBlacklistList();
	    0 < c.length && b.writeRepeatedString(5, c);
	    c = module$contents$jspb$Message_Message.getField(a, 13);
	    null != c && b.writeInt32(13, c);
	    c = a.getComputeSettings();
	    null != c && b.writeMessage(9, c, proto.tflite.proto.ComputeSettings.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.task.core.ExternalFile, 10)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setModelFileWithMetadata = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 10, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 10)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.hasModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 10)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 11, "en")
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setDisplayNamesLocale = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 11, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 11)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.hasDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 11)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getMaxResults = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2, -1)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setMaxResults = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearMaxResults = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.hasMaxResults = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getScoreThreshold = function () {
	    return module$contents$jspb$Message_Message.getFloatingPointFieldWithDefault(this, 3)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setScoreThreshold = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearScoreThreshold = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.hasScoreThreshold = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getClassNameWhitelistList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedField(this, 4)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setClassNameWhitelistList = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a || [])
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.addClassNameWhitelist = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedField(this, 4, a, b)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearClassNameWhitelistList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedField(this, 4)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getClassNameBlacklistList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedField(this, 5)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setClassNameBlacklistList = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 5, a || [])
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.addClassNameBlacklist = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedField(this, 5, a, b)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearClassNameBlacklistList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedField(this, 5)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getNumThreads = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 13, -1)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setNumThreads = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 13, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearNumThreads = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 13)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.hasNumThreads = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 13)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.getComputeSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.ComputeSettings, 9)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.setComputeSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 9, a)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.clearComputeSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 9)
	};
	proto.tflite.task.vision.ImageClassifierOptions.prototype.hasComputeSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 9)
	};
	proto.tflite.task.vision.ImageClassifierOptions.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.ImageClassifierOptions, a)
	};
	proto.tflite.task.vision.ImageSegmenterOptions = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.task.vision.ImageSegmenterOptions, module$contents$jspb$Message_Message);
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.ImageSegmenterOptions.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.ImageSegmenterOptions.toObject(a, this)
	}, proto.tflite.task.vision.ImageSegmenterOptions.toObject = function (a, b) {
	    var c, d = {
	        modelFileWithMetadata: (c = b.getModelFileWithMetadata()) && proto.tflite.task.core.ExternalFile.toObject(a, c),
	        displayNamesLocale: module$contents$jspb$Message_Message.getStringFieldWithDefault(b, 6, "en"),
	        outputType: module$contents$jspb$Message_Message.getFieldWithDefault(b,
	            3, 1),
	        numThreads: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b, 7, -1),
	        computeSettings: (c = b.getComputeSettings()) && proto.tflite.proto.ComputeSettings.toObject(a, c)
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.ImageSegmenterOptions.ObjectFormat = function () {}, proto.tflite.task.vision.ImageSegmenterOptions.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.ImageSegmenterOptions;
	    a.modelFileWithMetadata && module$contents$jspb$Message_Message.setWrapperField(b, 5, proto.tflite.task.core.ExternalFile.fromObject(a.modelFileWithMetadata));
	    null != a.displayNamesLocale && module$contents$jspb$Message_Message.setField(b, 6, a.displayNamesLocale);
	    null != a.outputType && module$contents$jspb$Message_Message.setField(b, 3, a.outputType);
	    null != a.numThreads && module$contents$jspb$Message_Message.setField(b, 7, a.numThreads);
	    a.computeSettings && module$contents$jspb$Message_Message.setWrapperField(b, 4, proto.tflite.proto.ComputeSettings.fromObject(a.computeSettings));
	    return b
	});
	proto.tflite.task.vision.ImageSegmenterOptions.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.ImageSegmenterOptions.deserializeBinaryFromReader(new proto.tflite.task.vision.ImageSegmenterOptions, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.ImageSegmenterOptions.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 5:
	            var c = new proto.tflite.task.core.ExternalFile;
	            b.readMessage(c, proto.tflite.task.core.ExternalFile.deserializeBinaryFromReader);
	            a.setModelFileWithMetadata(c);
	            break;
	        case 6:
	            c = b.readString();
	            a.setDisplayNamesLocale(c);
	            break;
	        case 3:
	            c = b.readEnum();
	            a.setOutputType(c);
	            break;
	        case 7:
	            c = b.readInt32();
	            a.setNumThreads(c);
	            break;
	        case 4:
	            c = new proto.tflite.proto.ComputeSettings;
	            b.readMessage(c, proto.tflite.proto.ComputeSettings.deserializeBinaryFromReader);
	            a.setComputeSettings(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.ImageSegmenterOptions.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.ImageSegmenterOptions.serializeBinaryToWriter = function (a, b) {
	    var c = a.getModelFileWithMetadata();
	    null != c && b.writeMessage(5, c, proto.tflite.task.core.ExternalFile.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 6);
	    null != c && b.writeString(6, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeEnum(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 7);
	    null != c && b.writeInt32(7, c);
	    c = a.getComputeSettings();
	    null != c && b.writeMessage(4, c,
	        proto.tflite.proto.ComputeSettings.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.ImageSegmenterOptions.OutputType = {
	    UNSPECIFIED: 0,
	    CATEGORY_MASK: 1,
	    CONFIDENCE_MASK: 2
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.getModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.task.core.ExternalFile, 5)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.setModelFileWithMetadata = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 5, a)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.clearModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 5)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.hasModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 5)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.getDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 6, "en")
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.setDisplayNamesLocale = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 6, a)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.clearDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 6)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.hasDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 6)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.getOutputType = function () {
	    return module$contents$jspb$Message_Message.getFieldWithDefault(this, 3, 1)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.setOutputType = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.clearOutputType = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.hasOutputType = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.getNumThreads = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 7, -1)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.setNumThreads = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 7, a)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.clearNumThreads = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 7)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.hasNumThreads = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 7)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.getComputeSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.ComputeSettings, 4)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.setComputeSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 4, a)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.clearComputeSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 4)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.prototype.hasComputeSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.vision.ImageSegmenterOptions.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.ImageSegmenterOptions, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.ObjectDetectorOptions.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.ObjectDetectorOptions, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.ObjectDetectorOptions.repeatedFields_ = [5, 6];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.ObjectDetectorOptions.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.ObjectDetectorOptions.toObject(a, this)
	}, proto.tflite.task.vision.ObjectDetectorOptions.toObject = function (a, b) {
	    var c, d = {
	        modelFileWithMetadata: (c = b.getModelFileWithMetadata()) && proto.tflite.task.core.ExternalFile.toObject(a, c),
	        displayNamesLocale: module$contents$jspb$Message_Message.getStringFieldWithDefault(b, 2, "en"),
	        maxResults: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b,
	            3, -1),
	        scoreThreshold: null == (c = module$contents$jspb$Message_Message.getOptionalFloatingPointField(b, 4)) ? void 0 : c,
	        classNameWhitelistList: null == (c = module$contents$jspb$Message_Message.getRepeatedField(b, 5)) ? void 0 : c,
	        classNameBlacklistList: null == (c = module$contents$jspb$Message_Message.getRepeatedField(b, 6)) ? void 0 : c,
	        numThreads: module$contents$jspb$Message_Message.getIntegerFieldWithDefault(b, 7, -1),
	        computeSettings: (c = b.getComputeSettings()) && proto.tflite.proto.ComputeSettings.toObject(a, c)
	    };
	    a && (d.$jspbMessageInstance =
	        b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.ObjectDetectorOptions.ObjectFormat = function () {}, proto.tflite.task.vision.ObjectDetectorOptions.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.ObjectDetectorOptions;
	    a.modelFileWithMetadata && module$contents$jspb$Message_Message.setWrapperField(b, 1, proto.tflite.task.core.ExternalFile.fromObject(a.modelFileWithMetadata));
	    null != a.displayNamesLocale && module$contents$jspb$Message_Message.setField(b, 2, a.displayNamesLocale);
	    null != a.maxResults && module$contents$jspb$Message_Message.setField(b, 3, a.maxResults);
	    null != a.scoreThreshold && module$contents$jspb$Message_Message.setField(b, 4, a.scoreThreshold);
	    null != a.classNameWhitelistList && module$contents$jspb$Message_Message.setField(b, 5, a.classNameWhitelistList);
	    null != a.classNameBlacklistList && module$contents$jspb$Message_Message.setField(b, 6, a.classNameBlacklistList);
	    null != a.numThreads && module$contents$jspb$Message_Message.setField(b, 7, a.numThreads);
	    a.computeSettings && module$contents$jspb$Message_Message.setWrapperField(b,
	        8, proto.tflite.proto.ComputeSettings.fromObject(a.computeSettings));
	    return b
	});
	proto.tflite.task.vision.ObjectDetectorOptions.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.ObjectDetectorOptions.deserializeBinaryFromReader(new proto.tflite.task.vision.ObjectDetectorOptions, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.ObjectDetectorOptions.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = new proto.tflite.task.core.ExternalFile;
	            b.readMessage(c, proto.tflite.task.core.ExternalFile.deserializeBinaryFromReader);
	            a.setModelFileWithMetadata(c);
	            break;
	        case 2:
	            c = b.readString();
	            a.setDisplayNamesLocale(c);
	            break;
	        case 3:
	            c = b.readInt32();
	            a.setMaxResults(c);
	            break;
	        case 4:
	            c = b.readFloat();
	            a.setScoreThreshold(c);
	            break;
	        case 5:
	            c = b.readString();
	            a.addClassNameWhitelist(c);
	            break;
	        case 6:
	            c = b.readString();
	            a.addClassNameBlacklist(c);
	            break;
	        case 7:
	            c = b.readInt32();
	            a.setNumThreads(c);
	            break;
	        case 8:
	            c = new proto.tflite.proto.ComputeSettings;
	            b.readMessage(c, proto.tflite.proto.ComputeSettings.deserializeBinaryFromReader);
	            a.setComputeSettings(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.ObjectDetectorOptions.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.ObjectDetectorOptions.serializeBinaryToWriter = function (a, b) {
	    var c = a.getModelFileWithMetadata();
	    null != c && b.writeMessage(1, c, proto.tflite.task.core.ExternalFile.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeString(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeInt32(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeFloat(4, c);
	    c = a.getClassNameWhitelistList();
	    0 < c.length && b.writeRepeatedString(5,
	        c);
	    c = a.getClassNameBlacklistList();
	    0 < c.length && b.writeRepeatedString(6, c);
	    c = module$contents$jspb$Message_Message.getField(a, 7);
	    null != c && b.writeInt32(7, c);
	    c = a.getComputeSettings();
	    null != c && b.writeMessage(8, c, proto.tflite.proto.ComputeSettings.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.task.core.ExternalFile, 1)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setModelFileWithMetadata = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 1, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 1)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.hasModelFileWithMetadata = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 2, "en")
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setDisplayNamesLocale = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.hasDisplayNamesLocale = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getMaxResults = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 3, -1)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setMaxResults = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearMaxResults = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.hasMaxResults = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getScoreThreshold = function () {
	    return module$contents$jspb$Message_Message.getFloatingPointFieldWithDefault(this, 4)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setScoreThreshold = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearScoreThreshold = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.hasScoreThreshold = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getClassNameWhitelistList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedField(this, 5)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setClassNameWhitelistList = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 5, a || [])
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.addClassNameWhitelist = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedField(this, 5, a, b)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearClassNameWhitelistList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedField(this, 5)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getClassNameBlacklistList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedField(this, 6)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setClassNameBlacklistList = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 6, a || [])
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.addClassNameBlacklist = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedField(this, 6, a, b)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearClassNameBlacklistList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedField(this, 6)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getNumThreads = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 7, -1)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setNumThreads = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 7, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearNumThreads = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 7)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.hasNumThreads = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 7)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.getComputeSettings = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.proto.ComputeSettings, 8)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.setComputeSettings = function (a) {
	    return module$contents$jspb$Message_Message.setWrapperField(this, 8, a)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.clearComputeSettings = function () {
	    return module$contents$jspb$Message_Message.clearWrapperField(this, 8)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.prototype.hasComputeSettings = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 8)
	};
	proto.tflite.task.vision.ObjectDetectorOptions.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.ObjectDetectorOptions, a)
	};
	proto.tflite.task.vision.Segmentation = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.Segmentation.repeatedFields_, proto.tflite.task.vision.Segmentation.oneofGroups_);
	};
	goog.inherits(proto.tflite.task.vision.Segmentation, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.Segmentation.ConfidenceMask = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.Segmentation.ConfidenceMask.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.Segmentation.ConfidenceMask, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.Segmentation.ConfidenceMasks = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.Segmentation.ConfidenceMasks.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.Segmentation.ConfidenceMasks, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.Segmentation.ColoredLabel = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, null, null);
	};
	goog.inherits(proto.tflite.task.vision.Segmentation.ColoredLabel, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.Segmentation.repeatedFields_ = [5];
	proto.tflite.task.vision.Segmentation.oneofGroups_ = [
	    [1, 4]
	];
	proto.tflite.task.vision.Segmentation.MaskOneofCase = {
	    MASK_ONEOF_NOT_SET: 0,
	    CATEGORY_MASK: 1,
	    CONFIDENCE_MASKS: 4
	};
	proto.tflite.task.vision.Segmentation.prototype.getMaskOneofCase = function () {
	    return module$contents$jspb$Message_Message.computeOneofCase(this, proto.tflite.task.vision.Segmentation.oneofGroups_[0])
	};
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Segmentation.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Segmentation.toObject(a, this)
	}, proto.tflite.task.vision.Segmentation.toObject = function (a, b) {
	    var c, d = {
	        categoryMask: b.getCategoryMask_asB64(),
	        confidenceMasks: (c = b.getConfidenceMasks()) && proto.tflite.task.vision.Segmentation.ConfidenceMasks.toObject(a, c),
	        width: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        height: null == (c =
	            module$contents$jspb$Message_Message.getField(b, 3)) ? void 0 : c,
	        coloredLabelsList: module$contents$jspb$Message_Message.toObjectList(b.getColoredLabelsList(), proto.tflite.task.vision.Segmentation.ColoredLabel.toObject, a)
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Segmentation.ObjectFormat = function () {}, proto.tflite.task.vision.Segmentation.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Segmentation;
	    null != a.categoryMask && module$contents$jspb$Message_Message.setOneofField(b, 1, proto.tflite.task.vision.Segmentation.oneofGroups_[0], a.categoryMask);
	    a.confidenceMasks && module$contents$jspb$Message_Message.setOneofWrapperField(b, 4, proto.tflite.task.vision.Segmentation.oneofGroups_[0],
	        proto.tflite.task.vision.Segmentation.ConfidenceMasks.fromObject(a.confidenceMasks));
	    null != a.width && module$contents$jspb$Message_Message.setField(b, 2, a.width);
	    null != a.height && module$contents$jspb$Message_Message.setField(b, 3, a.height);
	    a.coloredLabelsList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 5, a.coloredLabelsList.map(proto.tflite.task.vision.Segmentation.ColoredLabel.fromObject));
	    return b
	});
	proto.tflite.task.vision.Segmentation.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Segmentation.deserializeBinaryFromReader(new proto.tflite.task.vision.Segmentation, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Segmentation.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readBytes();
	            a.setCategoryMask(c);
	            break;
	        case 4:
	            c = new proto.tflite.task.vision.Segmentation.ConfidenceMasks;
	            b.readMessage(c, proto.tflite.task.vision.Segmentation.ConfidenceMasks.deserializeBinaryFromReader);
	            a.setConfidenceMasks(c);
	            break;
	        case 2:
	            c = b.readInt32();
	            a.setWidth(c);
	            break;
	        case 3:
	            c = b.readInt32();
	            a.setHeight(c);
	            break;
	        case 5:
	            c = new proto.tflite.task.vision.Segmentation.ColoredLabel;
	            b.readMessage(c, proto.tflite.task.vision.Segmentation.ColoredLabel.deserializeBinaryFromReader);
	            a.addColoredLabels(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Segmentation.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Segmentation.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Segmentation.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeBytes(1, c);
	    c = a.getConfidenceMasks();
	    null != c && b.writeMessage(4, c, proto.tflite.task.vision.Segmentation.ConfidenceMasks.serializeBinaryToWriter);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeInt32(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeInt32(3, c);
	    c = a.getColoredLabelsList();
	    0 < c.length && b.writeRepeatedMessage(5,
	        c, proto.tflite.task.vision.Segmentation.ColoredLabel.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.repeatedFields_ = [1];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Segmentation.ConfidenceMask.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Segmentation.ConfidenceMask.toObject(a, this)
	}, proto.tflite.task.vision.Segmentation.ConfidenceMask.toObject = function (a, b) {
	    var c, d = {
	        valueList: null == (c = module$contents$jspb$Message_Message.getRepeatedFloatingPointField(b, 1)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Segmentation.ConfidenceMask.ObjectFormat = function () {}, proto.tflite.task.vision.Segmentation.ConfidenceMask.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Segmentation.ConfidenceMask;
	    null != a.valueList && module$contents$jspb$Message_Message.setField(b, 1, a.valueList);
	    return b
	});
	proto.tflite.task.vision.Segmentation.ConfidenceMask.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Segmentation.ConfidenceMask.deserializeBinaryFromReader(new proto.tflite.task.vision.Segmentation.ConfidenceMask, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.isDelimited() ? b.readPackedFloat() : [b.readFloat()];
	            for (var d = 0; d < c.length; d++) a.addValue(c[d]);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Segmentation.ConfidenceMask.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.serializeBinaryToWriter = function (a, b) {
	    a = a.getValueList();
	    0 < a.length && b.writePackedFloat(1, a);
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.prototype.getValueList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedFloatingPointField(this, 1)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.prototype.setValueList = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a || [])
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.prototype.addValue = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedField(this, 1, a, b)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.prototype.clearValueList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedField(this, 1)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMask.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Segmentation.ConfidenceMask, a)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.repeatedFields_ = [1];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Segmentation.ConfidenceMasks.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Segmentation.ConfidenceMasks.toObject(a, this)
	}, proto.tflite.task.vision.Segmentation.ConfidenceMasks.toObject = function (a, b) {
	    var c = {
	        confidenceMaskList: module$contents$jspb$Message_Message.toObjectList(b.getConfidenceMaskList(), proto.tflite.task.vision.Segmentation.ConfidenceMask.toObject, a)
	    };
	    a && (c.$jspbMessageInstance = b);
	    return c
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Segmentation.ConfidenceMasks.ObjectFormat = function () {}, proto.tflite.task.vision.Segmentation.ConfidenceMasks.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Segmentation.ConfidenceMasks;
	    a.confidenceMaskList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 1, a.confidenceMaskList.map(proto.tflite.task.vision.Segmentation.ConfidenceMask.fromObject));
	    return b
	});
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Segmentation.ConfidenceMasks.deserializeBinaryFromReader(new proto.tflite.task.vision.Segmentation.ConfidenceMasks, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = new proto.tflite.task.vision.Segmentation.ConfidenceMask;
	            b.readMessage(c, proto.tflite.task.vision.Segmentation.ConfidenceMask.deserializeBinaryFromReader);
	            a.addConfidenceMask(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Segmentation.ConfidenceMasks.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.serializeBinaryToWriter = function (a, b) {
	    a = a.getConfidenceMaskList();
	    0 < a.length && b.writeRepeatedMessage(1, a, proto.tflite.task.vision.Segmentation.ConfidenceMask.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.prototype.getConfidenceMaskList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Segmentation.ConfidenceMask, 1)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.prototype.setConfidenceMaskList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 1, a)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.prototype.addConfidenceMask = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 1, a, proto.tflite.task.vision.Segmentation.ConfidenceMask, b)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.prototype.clearConfidenceMaskList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 1)
	};
	proto.tflite.task.vision.Segmentation.ConfidenceMasks.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Segmentation.ConfidenceMasks, a)
	};
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.Segmentation.ColoredLabel.toObject(a, this)
	}, proto.tflite.task.vision.Segmentation.ColoredLabel.toObject = function (a, b) {
	    var c, d = {
	        r: null == (c = module$contents$jspb$Message_Message.getField(b, 1)) ? void 0 : c,
	        g: null == (c = module$contents$jspb$Message_Message.getField(b, 2)) ? void 0 : c,
	        b: null == (c = module$contents$jspb$Message_Message.getField(b,
	            3)) ? void 0 : c,
	        className: null == (c = module$contents$jspb$Message_Message.getField(b, 4)) ? void 0 : c,
	        displayName: null == (c = module$contents$jspb$Message_Message.getField(b, 5)) ? void 0 : c
	    };
	    a && (d.$jspbMessageInstance = b);
	    return d
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.Segmentation.ColoredLabel.ObjectFormat = function () {}, proto.tflite.task.vision.Segmentation.ColoredLabel.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.Segmentation.ColoredLabel;
	    null != a.r && module$contents$jspb$Message_Message.setField(b, 1, a.r);
	    null != a.g && module$contents$jspb$Message_Message.setField(b, 2, a.g);
	    null != a.b && module$contents$jspb$Message_Message.setField(b, 3, a.b);
	    null != a.className && module$contents$jspb$Message_Message.setField(b,
	        4, a.className);
	    null != a.displayName && module$contents$jspb$Message_Message.setField(b, 5, a.displayName);
	    return b
	});
	proto.tflite.task.vision.Segmentation.ColoredLabel.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.Segmentation.ColoredLabel.deserializeBinaryFromReader(new proto.tflite.task.vision.Segmentation.ColoredLabel, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = b.readUint32();
	            a.setR(c);
	            break;
	        case 2:
	            c = b.readUint32();
	            a.setG(c);
	            break;
	        case 3:
	            c = b.readUint32();
	            a.setB(c);
	            break;
	        case 4:
	            c = b.readString();
	            a.setClassName(c);
	            break;
	        case 5:
	            c = b.readString();
	            a.setDisplayName(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.Segmentation.ColoredLabel.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.serializeBinaryToWriter = function (a, b) {
	    var c = module$contents$jspb$Message_Message.getField(a, 1);
	    null != c && b.writeUint32(1, c);
	    c = module$contents$jspb$Message_Message.getField(a, 2);
	    null != c && b.writeUint32(2, c);
	    c = module$contents$jspb$Message_Message.getField(a, 3);
	    null != c && b.writeUint32(3, c);
	    c = module$contents$jspb$Message_Message.getField(a, 4);
	    null != c && b.writeString(4, c);
	    c = module$contents$jspb$Message_Message.getField(a, 5);
	    null != c && b.writeString(5, c);
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.getR = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 1)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.setR = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 1, a)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.clearR = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 1)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.hasR = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.getG = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.setG = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.clearG = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.hasG = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.getB = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 3)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.setB = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.clearB = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.hasB = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.getClassName = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 4)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.setClassName = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 4, a)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.clearClassName = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 4)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.hasClassName = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.getDisplayName = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 5)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.setDisplayName = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 5, a)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.clearDisplayName = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 5)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.prototype.hasDisplayName = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 5)
	};
	proto.tflite.task.vision.Segmentation.ColoredLabel.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Segmentation.ColoredLabel, a)
	};
	proto.tflite.task.vision.Segmentation.prototype.getCategoryMask = function () {
	    return module$contents$jspb$Message_Message.getStringFieldWithDefault(this, 1)
	};
	proto.tflite.task.vision.Segmentation.prototype.getCategoryMask_asB64 = function () {
	    return module$contents$jspb$Message_Message.bytesAsB64(this.getCategoryMask())
	};
	proto.tflite.task.vision.Segmentation.prototype.getCategoryMask_asU8 = function () {
	    return module$contents$jspb$Message_Message.bytesAsU8(this.getCategoryMask())
	};
	proto.tflite.task.vision.Segmentation.prototype.setCategoryMask = function (a) {
	    return module$contents$jspb$Message_Message.setOneofField(this, 1, proto.tflite.task.vision.Segmentation.oneofGroups_[0], a)
	};
	proto.tflite.task.vision.Segmentation.prototype.clearCategoryMask = function () {
	    return module$contents$jspb$Message_Message.clearOneofField(this, 1, proto.tflite.task.vision.Segmentation.oneofGroups_[0])
	};
	proto.tflite.task.vision.Segmentation.prototype.hasCategoryMask = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 1)
	};
	proto.tflite.task.vision.Segmentation.prototype.getConfidenceMasks = function () {
	    return module$contents$jspb$Message_Message.getWrapperField(this, proto.tflite.task.vision.Segmentation.ConfidenceMasks, 4)
	};
	proto.tflite.task.vision.Segmentation.prototype.setConfidenceMasks = function (a) {
	    return module$contents$jspb$Message_Message.setOneofWrapperField(this, 4, proto.tflite.task.vision.Segmentation.oneofGroups_[0], a)
	};
	proto.tflite.task.vision.Segmentation.prototype.clearConfidenceMasks = function () {
	    return module$contents$jspb$Message_Message.clearOneofWrapperField(this, 4, proto.tflite.task.vision.Segmentation.oneofGroups_[0])
	};
	proto.tflite.task.vision.Segmentation.prototype.hasConfidenceMasks = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 4)
	};
	proto.tflite.task.vision.Segmentation.prototype.getWidth = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 2)
	};
	proto.tflite.task.vision.Segmentation.prototype.setWidth = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 2, a)
	};
	proto.tflite.task.vision.Segmentation.prototype.clearWidth = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 2)
	};
	proto.tflite.task.vision.Segmentation.prototype.hasWidth = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 2)
	};
	proto.tflite.task.vision.Segmentation.prototype.getHeight = function () {
	    return module$contents$jspb$Message_Message.getIntegerFieldWithDefault(this, 3)
	};
	proto.tflite.task.vision.Segmentation.prototype.setHeight = function (a) {
	    return module$contents$jspb$Message_Message.setField(this, 3, a)
	};
	proto.tflite.task.vision.Segmentation.prototype.clearHeight = function () {
	    return module$contents$jspb$Message_Message.clearField(this, 3)
	};
	proto.tflite.task.vision.Segmentation.prototype.hasHeight = function () {
	    return module$contents$jspb$Message_Message.hasField(this, 3)
	};
	proto.tflite.task.vision.Segmentation.prototype.getColoredLabelsList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Segmentation.ColoredLabel, 5)
	};
	proto.tflite.task.vision.Segmentation.prototype.setColoredLabelsList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 5, a)
	};
	proto.tflite.task.vision.Segmentation.prototype.addColoredLabels = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 5, a, proto.tflite.task.vision.Segmentation.ColoredLabel, b)
	};
	proto.tflite.task.vision.Segmentation.prototype.clearColoredLabelsList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 5)
	};
	proto.tflite.task.vision.Segmentation.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.Segmentation, a)
	};
	proto.tflite.task.vision.SegmentationResult = function (a) {
	    module$contents$jspb$Message_Message.initialize(this, a, 0, -1, proto.tflite.task.vision.SegmentationResult.repeatedFields_, null);
	};
	goog.inherits(proto.tflite.task.vision.SegmentationResult, module$contents$jspb$Message_Message);
	proto.tflite.task.vision.SegmentationResult.repeatedFields_ = [1];
	module$contents$jspb$Message_Message.GENERATE_TO_OBJECT && (proto.tflite.task.vision.SegmentationResult.prototype.toObject = function (a) {
	    return proto.tflite.task.vision.SegmentationResult.toObject(a, this)
	}, proto.tflite.task.vision.SegmentationResult.toObject = function (a, b) {
	    var c = {
	        segmentationList: module$contents$jspb$Message_Message.toObjectList(b.getSegmentationList(), proto.tflite.task.vision.Segmentation.toObject, a)
	    };
	    a && (c.$jspbMessageInstance = b);
	    return c
	});
	module$contents$jspb$Message_Message.GENERATE_FROM_OBJECT && (proto.tflite.task.vision.SegmentationResult.ObjectFormat = function () {}, proto.tflite.task.vision.SegmentationResult.fromObject = function (a) {
	    var b = new proto.tflite.task.vision.SegmentationResult;
	    a.segmentationList && module$contents$jspb$Message_Message.setRepeatedWrapperField(b, 1, a.segmentationList.map(proto.tflite.task.vision.Segmentation.fromObject));
	    return b
	});
	proto.tflite.task.vision.SegmentationResult.deserializeBinary = function (a) {
	    return proto.tflite.task.vision.SegmentationResult.deserializeBinaryFromReader(new proto.tflite.task.vision.SegmentationResult, new module$contents$jspb$BinaryReader_BinaryReader(a))
	};
	proto.tflite.task.vision.SegmentationResult.deserializeBinaryFromReader = function (a, b) {
	    for (; b.nextField() && !b.isEndGroup();) switch (b.getFieldNumber()) {
	        case 1:
	            var c = new proto.tflite.task.vision.Segmentation;
	            b.readMessage(c, proto.tflite.task.vision.Segmentation.deserializeBinaryFromReader);
	            a.addSegmentation(c);
	            break;
	        default:
	            b.skipField();
	    }
	    return a
	};
	proto.tflite.task.vision.SegmentationResult.prototype.serializeBinary = function () {
	    var a = new module$contents$jspb$BinaryWriter_BinaryWriter;
	    proto.tflite.task.vision.SegmentationResult.serializeBinaryToWriter(this, a);
	    return a.getResultBuffer()
	};
	proto.tflite.task.vision.SegmentationResult.serializeBinaryToWriter = function (a, b) {
	    a = a.getSegmentationList();
	    0 < a.length && b.writeRepeatedMessage(1, a, proto.tflite.task.vision.Segmentation.serializeBinaryToWriter);
	};
	proto.tflite.task.vision.SegmentationResult.prototype.getSegmentationList = function () {
	    return module$contents$jspb$Message_Message.getRepeatedWrapperField(this, proto.tflite.task.vision.Segmentation, 1)
	};
	proto.tflite.task.vision.SegmentationResult.prototype.setSegmentationList = function (a) {
	    return module$contents$jspb$Message_Message.setRepeatedWrapperField(this, 1, a)
	};
	proto.tflite.task.vision.SegmentationResult.prototype.addSegmentation = function (a, b) {
	    return module$contents$jspb$Message_Message.addToRepeatedWrapperField(this, 1, a, proto.tflite.task.vision.Segmentation, b)
	};
	proto.tflite.task.vision.SegmentationResult.prototype.clearSegmentationList = function () {
	    return module$contents$jspb$Message_Message.clearRepeatedWrapperField(this, 1)
	};
	proto.tflite.task.vision.SegmentationResult.deserialize = function (a) {
	    return module$contents$jspb$Message_Message.deserializeWithCtor(proto.tflite.task.vision.SegmentationResult, a)
	};
	var module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader = {},
	    module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader_WASM_SIMD_CHECK = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 0, 65, 0, 253, 15, 26, 11]),
	    module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader_WASM_MULTI_THREADED_CHECK = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0,
	        1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 5, 4, 1, 3, 1, 1, 10, 11, 1, 9, 0, 65, 0, 254, 16, 2, 0, 26, 11
	    ]);
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader = function (a, b, c) {
	    this.wasmModulesPath = a;
	    this.tfliteWebApiName = b;
	    this.wasmWebWorkerObjectURL = c;
	    this.scriptSrc = "";
	    this.wasmFeatures = {
	        simd: !1,
	        multiThreading: !1
	    };
	    this.scriptSrc = document.currentScript ? document.currentScript.getAttribute("src") || "" : "";
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance = function (a, b, c) {
	    var d = a + b;
	    c = URL.createObjectURL(new Blob([c], {
	        type: "application/javascript"
	    }));
	    module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.instances.has(d) || (a = new module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader(a,
	        b, c), module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.instances.set(d, a));
	    return module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.instances.get(d)
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.load = function (a) {
	    this.loadPromise || (this.loadPromise = this.loadModule(void 0 === a ? !1 : a));
	    return this.loadPromise
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.getWASMFeatures = function () {
	    return Object.assign({}, this.wasmFeatures)
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.loadModule = function (a) {
	    a = void 0 === a ? !1 : a;
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function c() {
	        var d = this,
	            e, g, h, k, l, m, n, r, q, p, t, u, x, w;
	        return $jscomp.generator.createGenerator(c, function (v) {
	            return 1 == v.nextAddress ? (e = d, v.yield(d.genFullLoaderUrl(), 2)) : 3 != v.nextAddress ? (g = v.yieldResult, g.startsWith("http") ||
	                (g.startsWith("/") ? g = d.scriptSrc.startsWith("http") ? "" + goog.uri.utils.getHost(d.scriptSrc) + g : "" + window.location.origin + g : a ? d.scriptSrc.startsWith("http") ? (h = d.scriptSrc.lastIndexOf("/"), k = d.scriptSrc.substring(0, h), g = k + "/" + g) : d.scriptSrc.startsWith("/") ? (l = d.scriptSrc.lastIndexOf("/"), m = d.scriptSrc.substring(0, l), g = "" + window.location.origin + m + "/" + g) : (n = window.location.href.lastIndexOf("/"), r = window.location.href.substring(0, n), q = d.scriptSrc.lastIndexOf("/"), 0 > q ? g = r + "/" + g : (p = d.scriptSrc.substring(0,
	                    q), g = r + "/" + p + "/" + g)) : (t = window.location.href, u = t.lastIndexOf("/"), x = t.substring(0, u), g = x + "/" + g)), w = !0, v.yield(d.loadScript(g).catch(function () {
	                    w = !1;
	                }), 3)) : w ? v.return(window[d.tfliteWebApiName + "_ModuleFactory"]({
	                mainScriptUrlOrBlob: g,
	                locateFile: function (y, z) {
	                    return y.endsWith(".worker.js") ? e.wasmWebWorkerObjectURL : z + y
	                }
	            })) : v.return(void 0)
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.genFullLoaderUrl = function () {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function b() {
	        var c = this,
	            d, e, f, g, h, k;
	        return $jscomp.generator.createGenerator(b, function (l) {
	            if (1 == l.nextAddress) return d = [c.wasmModulesPath, c.tfliteWebApiName, "_cc"], e = $jscomp, f = e.makeIterator, l.yield(Promise.all([c.isSupportedSimd(), c.isSupportedMultiThreaded()]),
	                2);
	            g = f.call(e, l.yieldResult);
	            h = g.next().value;
	            k = g.next().value;
	            c.wasmFeatures.simd = h;
	            c.wasmFeatures.multiThreading = k;
	            h && k ? d.push("_simd_threaded.js") : !h && k ? d.push("_threaded.js") : h && !k ? d.push("_simd.js") : d.push(".js");
	            return l.return(d.join(""))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.loadScript = function (a) {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function c() {
	        var d, e = this,
	            f;
	        return $jscomp.generator.createGenerator(c, function (g) {
	            if ("undefined" === typeof window) return importScripts(a), g.jumpTo(0);
	            d = document.createElement("script");
	            d.setAttribute("id", e.tfliteWebApiName + "_loader_script");
	            f = new Promise(function (h, k) {
	                d.onerror = function () {
	                    k();
	                    document.head.removeChild(d);
	                };
	                d.onload = h;
	            });
	            document.head.appendChild(d);
	            d.setAttribute("src", a);
	            return g.yield(f, 0)
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.isSupportedSimd = function () {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function b() {
	        return $jscomp.generator.createGenerator(b, function (c) {
	            switch (c.nextAddress) {
	                case 1:
	                    return c.setCatchFinallyBlocks(2), c.yield(WebAssembly.instantiate(module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader_WASM_SIMD_CHECK),
	                        4);
	                case 4:
	                    c.leaveTryBlock(3);
	                    break;
	                case 2:
	                    return c.enterCatchBlock(), c.return(!1);
	                case 3:
	                    return c.return(!0)
	            }
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.prototype.isSupportedMultiThreaded = function () {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function b() {
	        return $jscomp.generator.createGenerator(b, function (c) {
	            switch (c.nextAddress) {
	                case 1:
	                    return c.setCatchFinallyBlocks(2), (new MessageChannel).port1.postMessage(new SharedArrayBuffer(1)), c.yield(WebAssembly.instantiate(module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader_WASM_MULTI_THREADED_CHECK),
	                        4);
	                case 4:
	                    c.leaveTryBlock(3);
	                    break;
	                case 2:
	                    return c.enterCatchBlock(), c.return(!1);
	                case 3:
	                    return c.return(!0)
	            }
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.instances = new Map;

	function module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_convertCppVectorToArray(a) {
	    if (null == a) return [];
	    for (var b = [], c = 0; c < a.size(); c++) {
	        var d = a.get(c);
	        b.push(d);
	    }
	    return b
	}

	function module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(a, b) {
	    try {
	        return b(a)
	    } finally {
	        null != a && a.delete();
	    }
	}

	function module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callWithStatusOrAndDelete(a, b, c) {
	    try {
	        if (!a.ok()) throw Error(b + a.errorMessage());
	        return c(a.value())
	    } finally {
	        a.delete();
	    }
	}
	var module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext = null;

	function module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels(a) {
	    if (null == a) throw Error("pixels passed to fromPixels() can not be null");
	    var b = !1,
	        c = !1,
	        d = !1,
	        e = !1;
	    if ("undefined" !== typeof ImageData && a instanceof ImageData) b = !0;
	    else if ("undefined" !== typeof HTMLVideoElement && a instanceof HTMLVideoElement) c = !0;
	    else if ("undefined" !== typeof HTMLImageElement && a instanceof HTMLImageElement) d = !0;
	    else if (null != a.getContext) e = !0;
	    else throw Error("pixels passed to fromPixels() must be either an HTMLVideoElement, HTMLImageElement, HTMLCanvasElement, ImageData in browser, or OffscreenCanvas, ImageData in webworker or {data: Uint32Array, width: number, height: number}, but was " +
	        a.constructor.name);
	    if (c && c && 2 > a.readyState) throw Error("The video element has not loaded data yet. Please wait for `loadeddata` event on the <video> element.");
	    var f = $jscomp.makeIterator(c ? [a.videoWidth, a.videoHeight] : [a.width, a.height]),
	        g = f.next().value;
	    f = f.next().value;
	    if (e) var h = a.getContext("2d").getImageData(0, 0, g, f).data;
	    else if (b) h = a.data;
	    else if (d || c) null == module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext && (module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext =
	        document.createElement("canvas").getContext("2d")), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext.canvas.width = g, module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext.canvas.height = f, module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext.drawImage(a, 0, 0, g, f), h = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels2DContext.getImageData(0,
	        0, g, f).data;
	    return {
	        vals: h,
	        width: g,
	        height: f
	    }
	}var module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client = {},
	    module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath = "";

	function module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_setWasmPath(a) {
	    module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath = a;
	}
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.setWasmPath = module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_setWasmPath;
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.create = function (a) {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function c() {
	        var d, e, f, g, h, k, l, m, n, r, q;
	        return $jscomp.generator.createGenerator(c, function (p) {
	            if (1 == p.nextAddress) return d = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                e = $jscomp, f = e.makeIterator, p.yield(Promise.all([d.load(), fetch(a)]), 2);
	            if (3 != p.nextAddress) return g = f.call(e, p.yieldResult), h = g.next().value, k = g.next().value, l = Uint8Array, p.yield(k.arrayBuffer(), 3);
	            m = new l(p.yieldResult);
	            n = h._malloc(m.length);
	            h.HEAPU8.set(m, n);
	            r = d.getWASMFeatures();
	            q = h.BertNLClassifier.CreateFromBuffer(n, m.length);
	            if (null === q || void 0 === q || !q.ok()) throw Error("Failed to create BertNLClassifier: " + (null === q || void 0 === q ? void 0 : q.errorMessage()));
	            return p.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier(h,
	                [n], r, q.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.prototype.classify = function (a) {
	    if (null != this.cppClassifier) return a = this.cppClassifier.Classify(a), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(a, function (b) {
	        return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_convertCppVectorToArray(b)
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.BertNLClassifier", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier);
	goog.exportSymbol("tfweb.BertNLClassifier.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.create);
	goog.exportSymbol("tfweb.BertNLClassifier.prototype.classify", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.prototype.classify);
	goog.exportSymbol("tfweb.BertNLClassifier.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertNLClassifier.prototype.cleanUp);
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.create = function (a) {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function c() {
	        var d, e, f, g, h, k, l, m, n, r, q;
	        return $jscomp.generator.createGenerator(c, function (p) {
	            if (1 == p.nextAddress) return d = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                e = $jscomp, f = e.makeIterator, p.yield(Promise.all([d.load(), fetch(a)]), 2);
	            if (3 != p.nextAddress) return g = f.call(e, p.yieldResult), h = g.next().value, k = g.next().value, l = Uint8Array, p.yield(k.arrayBuffer(), 3);
	            m = new l(p.yieldResult);
	            n = h._malloc(m.length);
	            h.HEAPU8.set(m, n);
	            r = d.getWASMFeatures();
	            q = h.BertQuestionAnswerer.CreateFromBuffer(n, m.length);
	            if (null === q || void 0 === q || !q.ok()) throw Error("Failed to create BertQuestionAnswerer: " + (null === q || void 0 === q ? void 0 : q.errorMessage()));
	            return p.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer(h,
	                [n], r, q.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.prototype.answer = function (a, b) {
	    if (null != this.cppClassifier) return a = this.cppClassifier.Answer(a, b), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(a, function (c) {
	        return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_convertCppVectorToArray(c)
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.BertQuestionAnswerer", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer);
	goog.exportSymbol("tfweb.BertQuestionAnswerer.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.create);
	goog.exportSymbol("tfweb.BertQuestionAnswerer.prototype.answer", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.prototype.answer);
	goog.exportSymbol("tfweb.BertQuestionAnswerer.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.BertQuestionAnswerer.prototype.cleanUp);
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.create = function (a, b) {
	    b = void 0 === b ? new proto.tflite.task.vision.ImageClassifierOptions : b;
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function d() {
	        var e, f, g, h, k, l, m, n, r, q, p;
	        return $jscomp.generator.createGenerator(d, function (t) {
	            if (1 == t.nextAddress) return e = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                f = $jscomp, g = f.makeIterator, t.yield(Promise.all([e.load(), fetch(a)]), 2);
	            if (3 != t.nextAddress) return h = g.call(f, t.yieldResult), k = h.next().value, l = h.next().value, m = Uint8Array, t.yield(l.arrayBuffer(), 3);
	            n = new m(t.yieldResult);
	            r = k._malloc(n.length);
	            k.HEAPU8.set(n, r);
	            q = e.getWASMFeatures();
	            p = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(new k.ImageClassifierOptionsCppProto, function (u) {
	                if (null != u) return u.initFromJsProto(b), k.ImageClassifier.CreateFromOptions(r,
	                    n.length, u)
	            });
	            if (null === p || void 0 === p || !p.ok()) throw Error("Failed to create ImageClassifier: " + (null === p || void 0 === p ? void 0 : p.errorMessage()));
	            return t.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier(k, [r], q, p.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.prototype.classify = function (a) {
	    if (null != this.cppClassifier) {
	        var b = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels(a);
	        a = b.vals;
	        var c = b.width,
	            d = b.height;
	        if (null != a) return b = this.module._malloc(a.length), this.module.HEAPU8.set(a, b), a = this.cppClassifier.Classify(b, c, d), this.module._free(b), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callWithStatusOrAndDelete(a,
	            "Failed to run inference method classify: ",
	            function (e) {
	                e = e.toArrayBuffer();
	                return proto.tflite.task.vision.ClassificationResult.deserializeBinary(e)
	            })
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.ClassificationResult", proto.tflite.task.vision.ClassificationResult);
	goog.exportSymbol("tfweb.ClassificationResult.prototype.serializeBinary", proto.tflite.task.vision.ClassificationResult.prototype.serializeBinary);
	goog.exportSymbol("tfweb.ImageClassifierOptions", proto.tflite.task.vision.ImageClassifierOptions);
	goog.exportSymbol("tfweb.ImageClassifierOptions.prototype.serializeBinary", proto.tflite.task.vision.ImageClassifierOptions.prototype.serializeBinary);
	goog.exportSymbol("tfweb.ImageClassifier", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier);
	goog.exportSymbol("tfweb.ImageClassifier.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.create);
	goog.exportSymbol("tfweb.ImageClassifier.prototype.classify", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.prototype.classify);
	goog.exportSymbol("tfweb.ImageClassifier.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageClassifier.prototype.cleanUp);
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.create = function (a, b) {
	    b = void 0 === b ? new proto.tflite.task.vision.ImageSegmenterOptions : b;
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function d() {
	        var e, f, g, h, k, l, m, n, r, q, p;
	        return $jscomp.generator.createGenerator(d, function (t) {
	            if (1 == t.nextAddress) return e = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                f = $jscomp, g = f.makeIterator, t.yield(Promise.all([e.load(), fetch(a)]), 2);
	            if (3 != t.nextAddress) return h = g.call(f, t.yieldResult), k = h.next().value, l = h.next().value, m = Uint8Array, t.yield(l.arrayBuffer(), 3);
	            n = new m(t.yieldResult);
	            r = k._malloc(n.length);
	            k.HEAPU8.set(n, r);
	            q = e.getWASMFeatures();
	            p = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(new k.ImageSegmenterOptionsCppProto, function (u) {
	                if (null != u) return u.initFromJsProto(b), k.ImageSegmenter.CreateFromOptions(r,
	                    n.length, u)
	            });
	            if (null === p || void 0 === p || !p.ok()) throw Error("Failed to create ImageSegmenter: " + (null === p || void 0 === p ? void 0 : p.errorMessage()));
	            return t.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter(k, [r], q, p.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.prototype.segment = function (a) {
	    if (null != this.cppClassifier) {
	        var b = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels(a);
	        a = b.vals;
	        var c = b.width,
	            d = b.height;
	        if (null != a) return b = this.module._malloc(a.length), this.module.HEAPU8.set(a, b), a = this.cppClassifier.Segment(b, c, d), this.module._free(b), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callWithStatusOrAndDelete(a,
	            "Failed to run inference method segment: ",
	            function (e) {
	                e = e.toArrayBuffer();
	                return proto.tflite.task.vision.SegmentationResult.deserializeBinary(e)
	            })
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.ImageSegmenterOptions", proto.tflite.task.vision.ImageSegmenterOptions);
	goog.exportSymbol("tfweb.ImageSegmenterOptions.prototype.serializeBinary", proto.tflite.task.vision.ImageSegmenterOptions.prototype.serializeBinary);
	goog.exportSymbol("tfweb.SegmentationResult", proto.tflite.task.vision.SegmentationResult);
	goog.exportSymbol("tfweb.SegmentationResult.prototype.serializeBinary", proto.tflite.task.vision.SegmentationResult.prototype.serializeBinary);
	goog.exportSymbol("tfweb.ImageSegmenter", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter);
	goog.exportSymbol("tfweb.ImageSegmenter.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.create);
	goog.exportSymbol("tfweb.ImageSegmenter.prototype.segment", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.prototype.segment);
	goog.exportSymbol("tfweb.ImageSegmenter.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ImageSegmenter.prototype.cleanUp);
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.create = function (a, b) {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function d() {
	        var e, f, g, h, k, l, m, n, r, q, p;
	        return $jscomp.generator.createGenerator(d, function (t) {
	            if (1 == t.nextAddress) return e = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                f = $jscomp, g = f.makeIterator, t.yield(Promise.all([e.load(), fetch(a)]), 2);
	            if (3 != t.nextAddress) return h = g.call(f, t.yieldResult), k = h.next().value, l = h.next().value, m = Uint8Array, t.yield(l.arrayBuffer(), 3);
	            n = new m(t.yieldResult);
	            r = k._malloc(n.length);
	            k.HEAPU8.set(n, r);
	            q = e.getWASMFeatures();
	            p = k.NLClassifier.CreateFromBufferAndOptions(r, n.length, b);
	            if (null === p || void 0 === p || !p.ok()) throw Error("Failed to create NLClassifier: " + (null === p || void 0 === p ? void 0 : p.errorMessage()));
	            return t.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier(k,
	                [r], q, p.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.prototype.classify = function (a) {
	    if (null != this.cppClassifier) return a = this.cppClassifier.Classify(a), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(a, function (b) {
	        return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_convertCppVectorToArray(b)
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.NLClassifier", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier);
	goog.exportSymbol("tfweb.NLClassifier.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.create);
	goog.exportSymbol("tfweb.NLClassifier.prototype.classify", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.prototype.classify);
	goog.exportSymbol("tfweb.NLClassifier.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.NLClassifier.prototype.cleanUp);
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.create = function (a, b) {
	    b = void 0 === b ? new proto.tflite.task.vision.ObjectDetectorOptions : b;
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function d() {
	        var e, f, g, h, k, l, m, n, r, q, p;
	        return $jscomp.generator.createGenerator(d, function (t) {
	            if (1 == t.nextAddress) return e = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                f = $jscomp, g = f.makeIterator, t.yield(Promise.all([e.load(), fetch(a)]), 2);
	            if (3 != t.nextAddress) return h = g.call(f, t.yieldResult), k = h.next().value, l = h.next().value, m = Uint8Array, t.yield(l.arrayBuffer(), 3);
	            n = new m(t.yieldResult);
	            r = k._malloc(n.length);
	            k.HEAPU8.set(n, r);
	            q = e.getWASMFeatures();
	            p = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(new k.ObjectDetectorOptionsCppProto, function (u) {
	                if (null != u) return u.initFromJsProto(b), k.ObjectDetector.CreateFromOptions(r,
	                    n.length, u)
	            });
	            if (null === p || void 0 === p || !p.ok()) throw Error("Failed to create ObjectDetector: " + (null === p || void 0 === p ? void 0 : p.errorMessage()));
	            return t.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector(k, [r], q, p.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.prototype.detect = function (a) {
	    if (null != this.cppClassifier) {
	        var b = module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_fromPixels(a);
	        a = b.vals;
	        var c = b.width,
	            d = b.height;
	        if (null != a) return b = this.module._malloc(a.length), this.module.HEAPU8.set(a, b), a = this.cppClassifier.Detect(b, c, d), this.module._free(b), module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callWithStatusOrAndDelete(a,
	            "Failed to run inference method detect: ",
	            function (e) {
	                e = e.toArrayBuffer();
	                return proto.tflite.task.vision.DetectionResult.deserializeBinary(e)
	            })
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.DetectionResult", proto.tflite.task.vision.DetectionResult);
	goog.exportSymbol("tfweb.DetectionResult.prototype.serializeBinary", proto.tflite.task.vision.DetectionResult.prototype.serializeBinary);
	goog.exportSymbol("tfweb.ObjectDetectorOptions", proto.tflite.task.vision.ObjectDetectorOptions);
	goog.exportSymbol("tfweb.ObjectDetectorOptions.prototype.serializeBinary", proto.tflite.task.vision.ObjectDetectorOptions.prototype.serializeBinary);
	goog.exportSymbol("tfweb.ObjectDetector", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector);
	goog.exportSymbol("tfweb.ObjectDetector.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.create);
	goog.exportSymbol("tfweb.ObjectDetector.prototype.detect", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.prototype.detect);
	goog.exportSymbol("tfweb.ObjectDetector.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.ObjectDetector.prototype.cleanUp);
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner = function (a, b, c, d) {
	    this.module = a;
	    this.memOffsetsToFree = b;
	    this.wasmFeatures = c;
	    this.cppClassifier = d;
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.create = function (a, b) {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function d() {
	        var e, f, g, h, k, l, m, n, r, q, p;
	        return $jscomp.generator.createGenerator(d, function (t) {
	            if (1 == t.nextAddress) return e = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                f = $jscomp, g = f.makeIterator, t.yield(Promise.all([e.load(), fetch(a)]), 2);
	            if (3 != t.nextAddress) return h = g.call(f, t.yieldResult), k = h.next().value, l = h.next().value, m = Uint8Array, t.yield(l.arrayBuffer(), 3);
	            n = new m(t.yieldResult);
	            r = k._malloc(n.length);
	            k.HEAPU8.set(n, r);
	            q = e.getWASMFeatures();
	            p = k.TFLiteWebModelRunner.CreateFromBufferAndOptions(r, n.length, b);
	            if (null === p || void 0 === p || !p.ok()) throw Error("Failed to create TFLiteWebModelRunner: " + (null === p || void 0 === p ? void 0 : p.errorMessage()));
	            return t.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner(k,
	                [r], q, p.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.createFromBuffer = function (model_int_array, b) {
	    return (0, module$exports$google3$third_party$javascript$tslib$tslib.__awaiter)(this, void 0, void 0, function d() {
	        var e, f, g, h, k, l, n, r, q, p;
	        return $jscomp.generator.createGenerator(d, function (t) {
	            if (1 == t.nextAddress) return e = module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance(module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_wasmModulesPath,
	                    "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n'),
	                f = $jscomp, g = f.makeIterator, t.yield(Promise.all([e.load(), Promise.resolve()]), 2);
	            if (3 != t.nextAddress) return h = g.call(f, t.yieldResult), k = h.next().value, l = h.next().value, t.yield(l.arrayBuffer(), 3);
	            n = model_int_array;
	            r = k._malloc(n.length);
	            k.HEAPU8.set(n, r);
	            q = e.getWASMFeatures();
	            p = k.TFLiteWebModelRunner.CreateFromBufferAndOptions(r, n.length, b);
	            if (null === p || void 0 === p || !p.ok()) throw Error("Failed to create TFLiteWebModelRunner: " + (null === p || void 0 === p ? void 0 : p.errorMessage()));
	            return t.return(new module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner(k,
	                [r], q, p.value()))
	        })
	    })
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.getInputs = function () {
	    if (null != this.cppClassifier) {
	        var a = this.cppClassifier.GetInputs();
	        return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(a, function (b) {
	            return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_convertCppVectorToArray(b)
	        })
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.getOutputs = function () {
	    if (null != this.cppClassifier) {
	        var a = this.cppClassifier.GetOutputs();
	        return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_callAndDelete(a, function (b) {
	            return module$contents$google3$third_party$tensorflow_lite_support$web$task$codegen$common$utils_convertCppVectorToArray(b)
	        })
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.infer = function () {
	    if (null != this.cppClassifier) return this.cppClassifier.Infer()
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.cleanUp = function () {
	    if (null != this.cppClassifier) {
	        for (var a = $jscomp.makeIterator(this.memOffsetsToFree), b = a.next(); !b.done; b = a.next()) this.module._free(b.value);
	        this.cppClassifier.delete();
	    }
	};
	module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.getWASMFeatures = function () {
	    return this.wasmFeatures
	};
	goog.exportSymbol("tfweb.TFLiteWebModelRunner", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner);
	goog.exportSymbol("tfweb.TFLiteWebModelRunner.create", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.create);
	goog.exportSymbol("tfweb.TFLiteWebModelRunner.createFromBuffer", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.createFromBuffer);
	goog.exportSymbol("tfweb.TFLiteWebModelRunner.prototype.getInputs", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.getInputs);
	goog.exportSymbol("tfweb.TFLiteWebModelRunner.prototype.getOutputs", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.getOutputs);
	goog.exportSymbol("tfweb.TFLiteWebModelRunner.prototype.infer", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.infer);
	goog.exportSymbol("tfweb.TFLiteWebModelRunner.prototype.cleanUp", module$exports$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client.TFLiteWebModelRunner.prototype.cleanUp);
	goog.exportSymbol("tfweb.tflite_web_api.setWasmPath", module$contents$google3$third_party$tensorflow_lite_support$web$tflite_web_api_client_setWasmPath);
	module$exports$google3$third_party$tensorflow_lite_support$web$task$codegen$common$emscripten_module_loader.EmscriptenModuleLoader.getInstance("", "tflite_web_api", 'var initializedJS=false;var Module={};function threadPrintErr(){var text=Array.prototype.slice.call(arguments).join(" ");console.error(text)}function threadAlert(){var text=Array.prototype.slice.call(arguments).join(" ");postMessage({cmd:"alert",text:text,threadId:Module["_pthread_self"]()})}var err=threadPrintErr;this.alert=threadAlert;Module["instantiateWasm"]=function(info,receiveInstance){var instance=new WebAssembly.Instance(Module["wasmModule"],info);receiveInstance(instance);Module["wasmModule"]=null;return instance.exports};function moduleLoaded(){}this.onmessage=function(e){try{if(e.data.cmd==="load"){Module["wasmModule"]=e.data.wasmModule;Module["wasmMemory"]=e.data.wasmMemory;Module["buffer"]=Module["wasmMemory"].buffer;Module["ENVIRONMENT_IS_PTHREAD"]=true;if(typeof e.data.urlOrBlob==="string"){importScripts(e.data.urlOrBlob)}else{var objectUrl=URL.createObjectURL(e.data.urlOrBlob);importScripts(objectUrl);URL.revokeObjectURL(objectUrl)}tflite_web_api_ModuleFactory(Module).then(function(instance){Module=instance;moduleLoaded()})}else if(e.data.cmd==="objectTransfer"){Module["PThread"].receiveObjectTransfer(e.data)}else if(e.data.cmd==="run"){Module["__performance_now_clock_drift"]=performance.now()-e.data.time;Module["__emscripten_thread_init"](e.data.threadInfoStruct,0,0);var max=e.data.stackBase;var top=e.data.stackBase+e.data.stackSize;Module["establishStackSpace"](top,max);Module["PThread"].receiveObjectTransfer(e.data);Module["PThread"].threadInit();if(!initializedJS){Module["___embind_register_native_and_builtin_types"]();initializedJS=true}try{var result=Module["invokeEntryPoint"](e.data.start_routine,e.data.arg);if(Module["keepRuntimeAlive"]()){Module["PThread"].setExitStatus(result)}else{Module["PThread"].threadExit(result)}}catch(ex){if(ex==="Canceled!"){Module["PThread"].threadCancel()}else if(ex!="unwind"){if(ex instanceof Module["ExitStatus"]){if(Module["keepRuntimeAlive"]()){}else{Module["PThread"].threadExit(ex.status)}}else{Module["PThread"].threadExit(-2);throw ex}}}}else if(e.data.cmd==="cancel"){if(Module["_pthread_self"]()){Module["PThread"].threadCancel()}}else if(e.data.target==="setimmediate"){}else if(e.data.cmd==="processThreadQueue"){if(Module["_pthread_self"]()){Module["_emscripten_current_thread_process_queued_calls"]()}}else{err("worker.js received unknown command "+e.data.cmd);err(e.data)}}catch(ex){err("worker.js onmessage() captured an uncaught exception: "+ex);if(ex&&ex.stack)err(ex.stack);throw ex}};\n').load(!0);

	var tflite_web_api_client = {

	};

	var tfliteWebAPIClient = /*#__PURE__*/Object.freeze({
		__proto__: null,
		'default': tflite_web_api_client
	});

	/**
	 * @license
	 * Copyright 2021 Google LLC. All Rights Reserved.
	 * Licensed under the Apache License, Version 2.0 (the "License");
	 * you may not use this file except in compliance with the License.
	 * You may obtain a copy of the License at
	 *
	 * http://www.apache.org/licenses/LICENSE-2.0
	 *
	 * Unless required by applicable law or agreed to in writing, software
	 * distributed under the License is distributed on an "AS IS" BASIS,
	 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	 * See the License for the specific language governing permissions and
	 * limitations under the License.
	 * =============================================================================
	 */
	const DEFAULT_TFLITE_MODEL_RUNNER_OPTIONS = {
	    numThreads: -1,
	};
	const TFHUB_SEARCH_PARAM = '?lite-format=tflite';
	/**
	 * A `tf.TFLiteModel` is built from a TFLite model flatbuffer and its
	 * corresponding Interpreter.
	 *
	 * @doc {heading: 'Models', subheading: 'Classes'}
	 */
	class TFLiteModel {
	    constructor(modelRunner) {
	        this.modelRunner = modelRunner;
	    }
	    get inputs() {
	        const modelInputs = this.modelRunner.getInputs();
	        return this.convertTFLiteTensorInfos(modelInputs);
	    }
	    get outputs() {
	        const modelOutputs = this.modelRunner.getOutputs();
	        return this.convertTFLiteTensorInfos(modelOutputs);
	    }
	    /**
	     * Execute the inference for the input tensors.
	     *
	     * @param input The input tensors, when there is single input for the model,
	     * inputs param should be a Tensor. For models with multiple inputs, inputs
	     * params should be in either Tensor[] if the input order is fixed, or
	     * otherwise NamedTensorMap format.
	     *
	     * @param config Prediction configuration for specifying the batch size.
	     *     Currently this field is not used, and batch inference is not supported.
	     *
	     * @returns Inference result tensors. The output would be single Tensor if
	     * model has single output node, otherwise NamedTensorMap will be returned for
	     * model with multiple outputs. Tensor[] is not used.
	     *
	     * @doc {heading: 'Models', subheading: 'TFLiteModel'}
	     */
	    predict(inputs, config) {
	        const modelInputs = this.modelRunner.getInputs();
	        const modelOutputs = this.modelRunner.getOutputs();
	        // Set model inputs from the given tensors.
	        // A single tensor or a tensor array.
	        if (inputs instanceof tfjsCore.Tensor || Array.isArray(inputs)) {
	            let inputTensors;
	            if (inputs instanceof tfjsCore.Tensor) {
	                inputTensors = [inputs];
	            }
	            else {
	                inputTensors = inputs;
	            }
	            if (modelInputs.length !== inputTensors.length) {
	                throw new Error(`The size of TFLite model inputs (${modelInputs
                    .length}) does not match the size of the input tensors (${inputTensors.length})`);
	            }
	            for (let i = 0; i < modelInputs.length; i++) {
	                this.setModelInputFromTensor(modelInputs[i], inputTensors[i]);
	            }
	        }
	        // Named tensors.
	        else {
	            const inputTensorNames = Object.keys(inputs);
	            const modelInputMap = {};
	            modelInputs.forEach(modelInput => {
	                modelInputMap[modelInput.name] = modelInput;
	            });
	            const modelInputNames = Object.keys(modelInputMap);
	            this.checkMapInputs(inputTensorNames, modelInputNames);
	            for (const name of inputTensorNames) {
	                this.setModelInputFromTensor(modelInputMap[name], inputs[name]);
	            }
	        }
	        // Run inference.
	        const success = this.modelRunner.infer();
	        if (!success) {
	            throw new Error('Failed running inference');
	        }
	        // Convert model outputs to tensors.
	        const outputTensors = {};
	        for (let i = 0; i < modelOutputs.length; i++) {
	            const modelOutput = modelOutputs[i];
	            const outputTensor = tfjsCore.tensor(modelOutput.data(), this.getShapeFromTFLiteTensorInfo(modelOutput));
	            outputTensors[modelOutput.name] = outputTensor;
	        }
	        const names = Object.keys(outputTensors);
	        return names.length === 1 ? outputTensors[names[0]] : outputTensors;
	    }
	    /**
	     * Execute the inference for the input tensors and return activation
	     * values for specified output node names without batching.
	     *
	     * @param input The input tensors, when there is single input for the model,
	     * inputs param should be a Tensor. For models with multiple inputs, inputs
	     * params should be in either Tensor[] if the input order is fixed, or
	     * otherwise NamedTensorMap format.
	     *
	     * @param outputs string|string[]. List of output node names to retrieve
	     * activation from.
	     *
	     * @returns Activation values for the output nodes result tensors. The return
	     * type matches specified parameter outputs type. The output would be single
	     * Tensor if single output is specified, otherwise Tensor[] for multiple
	     * outputs.
	     *
	     * @doc {heading: 'Models', subheading: 'TFLiteModel'}
	     */
	    execute(inputs, outputs) {
	        throw new Error('execute() of TFLiteModel is not supported yet.');
	    }
	    setModelInputFromTensor(modelInput, tensor) {
	        // String and complex tensors are not supported.
	        if (tensor.dtype === 'string' || tensor.dtype === 'complex64') {
	            throw new Error(`Data type '${tensor.dtype}' not supported.`);
	        }
	        // Check shape.
	        //
	        // At this point, we've already checked that input tensors and model inputs
	        // have the same size.
	        const modelInputShape = modelInput.shape.split(',').map(dim => Number(dim));
	        if (!tensor.shape.every((dim, index) => modelInputShape[index] === -1 ||
	            modelInputShape[index] === dim)) {
	            throw new Error(`Input tensor shape mismatch: expect '${modelInput.shape}', got '${tensor.shape.join(',')}'.`);
	        }
	        // Check types.
	        switch (modelInput.dataType) {
	            // All 'bool' and 'int' tflite types accpet 'bool' or 'int32' tfjs types.
	            // Will throw error for 'float32' tfjs type.
	            case 'bool':
	            case 'int8':
	            case 'uint8':
	            case 'int16':
	            case 'uint32':
	            case 'int32':
	                if (tensor.dtype === 'float32') {
	                    throw this.getDataTypeMismatchError(modelInput.dataType, tensor.dtype);
	                }
	                else if (modelInput.dataType !== tensor.dtype) {
	                    console.warn(`WARNING: converting '${tensor.dtype}' to '${modelInput.dataType}'`);
	                }
	                break;
	            // All 'float' tflite types accept all tfjs types.
	            case 'float32':
	            case 'float64':
	                if (modelInput.dataType !== tensor.dtype) {
	                    console.warn(`WARNING: converting '${tensor.dtype}' to '${modelInput.dataType}'`);
	                }
	                break;
	        }
	        const modelInputBuffer = modelInput.data();
	        switch (modelInput.dataType) {
	            case 'int8':
	                modelInputBuffer.set(Int8Array.from(tensor.dataSync()));
	                break;
	            case 'uint8':
	            case 'bool':
	                modelInputBuffer.set(Uint8Array.from(tensor.dataSync()));
	                break;
	            case 'int16':
	                modelInputBuffer.set(Int16Array.from(tensor.dataSync()));
	                break;
	            case 'int32':
	                modelInputBuffer.set(Int32Array.from(tensor.dataSync()));
	                break;
	            case 'uint32':
	                modelInputBuffer.set(Uint32Array.from(tensor.dataSync()));
	                break;
	            case 'float32':
	                modelInputBuffer.set(Float32Array.from(tensor.dataSync()));
	                break;
	            case 'float64':
	                modelInputBuffer.set(Float64Array.from(tensor.dataSync()));
	                break;
	        }
	    }
	    convertTFLiteTensorInfos(infos) {
	        return infos.map(info => {
	            const dtype = getDTypeFromTFLiteType(info.dataType);
	            return {
	                name: info.name,
	                shape: this.getShapeFromTFLiteTensorInfo(info),
	                dtype,
	            };
	        });
	    }
	    checkMapInputs(inputTensorNames, modelInputNames) {
	        const notInModel = inputTensorNames.filter(name => !modelInputNames.includes(name));
	        const notInInput = modelInputNames.filter(name => !inputTensorNames.includes(name));
	        if (notInModel.length === 0 && notInInput.length === 0) {
	            return;
	        }
	        const msgParts = ['The model input names don\'t match the model input names.'];
	        if (notInModel.length > 0) {
	            msgParts.push(`Names in input but missing in model: [${notInModel}].`);
	        }
	        if (notInInput.length > 0) {
	            msgParts.push(`Names in model but missing in inputs: [${notInInput}].`);
	        }
	        throw new Error(msgParts.join(' '));
	    }
	    getShapeFromTFLiteTensorInfo(info) {
	        return info.shape.split(',').map(s => Number(s));
	    }
	    getDataTypeMismatchError(expected, got) {
	        return new Error(`Data type mismatch: input tensor expects '${expected}', got '${got}'`);
	    }
	}
	/**
	 * Loads a TFLiteModel from the given model url.
	 *
	 * @param modelUrl The path to the model.
	 * @param options Options related to model inference.
	 *
	 * @doc {heading: 'Models', subheading: 'TFLiteModel'}
	 */
	async function loadTFLiteModel(modelUrl, options = DEFAULT_TFLITE_MODEL_RUNNER_OPTIONS) {
	    // Handle tfhub links.
	    if (modelUrl.includes('tfhub.dev')) {
	        if (!modelUrl.endsWith(TFHUB_SEARCH_PARAM)) {
	            modelUrl = `${modelUrl}${TFHUB_SEARCH_PARAM}`;
	        }
	    }
	    const tfliteModelRunner = await undefined(modelUrl, options);
	    return new TFLiteModel(tfliteModelRunner);
	}
	async function loadTFLiteModelFromBuffer(buffer, options = DEFAULT_TFLITE_MODEL_RUNNER_OPTIONS) {
	    const tfliteModelRunner = await undefined(buffer, options);
	    return new TFLiteModel(tfliteModelRunner);
	}
	/** Returns the compatible tfjs DataType from the given TFLite data type. */
	function getDTypeFromTFLiteType(tfliteType) {
	    let dtype;
	    switch (tfliteType) {
	        case 'float32':
	        case 'float64':
	            dtype = 'float32';
	            break;
	        case 'int8':
	        case 'uint8':
	        case 'int16':
	        case 'int32':
	        case 'uint32':
	            dtype = 'int32';
	            break;
	        case 'bool':
	            dtype = 'bool';
	            break;
	    }
	    return dtype;
	}

	exports.TFLiteModel = TFLiteModel;
	exports.getDTypeFromTFLiteType = getDTypeFromTFLiteType;
	exports.loadTFLiteModel = loadTFLiteModel;
	exports.loadTFLiteModelFromBuffer = loadTFLiteModelFromBuffer;

	Object.defineProperty(exports, '__esModule', { value: true });

})));
//# sourceMappingURL=tf-tflite.es2017.js.map
