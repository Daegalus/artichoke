(function() {var implementors = {};
implementors["artichoke_backend"] = [{"text":"impl Extend&lt;mrb_value&gt; for Array","synthetic":false,"types":[]},{"text":"impl Extend&lt;Value&gt; for Array","synthetic":false,"types":[]}];
implementors["nix"] = [{"text":"impl Extend&lt;AtFlags&gt; for AtFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;OFlag&gt; for OFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;SealFlag&gt; for SealFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;SpliceFFlags&gt; for SpliceFFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;FallocateFlags&gt; for FallocateFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;ModuleInitFlags&gt; for ModuleInitFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;DeleteModuleFlags&gt; for DeleteModuleFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MntFlags&gt; for MntFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MQ_OFlag&gt; for MQ_OFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;FdFlag&gt; for FdFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;InterfaceFlags&gt; for InterfaceFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;PollFlags&gt; for PollFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;CloneFlags&gt; for CloneFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;EpollFlags&gt; for EpollFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;EpollCreateFlags&gt; for EpollCreateFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;EfdFlags&gt; for EfdFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MemFdCreateFlag&gt; for MemFdCreateFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;ProtFlags&gt; for ProtFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MapFlags&gt; for MapFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MsFlags&gt; for MsFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;MlockAllFlags&gt; for MlockAllFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;Options&gt; for Options","synthetic":false,"types":[]},{"text":"impl Extend&lt;QuotaValidFlags&gt; for QuotaValidFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SaFlags&gt; for SaFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SfdFlags&gt; for SfdFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SockFlag&gt; for SockFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;MsgFlags&gt; for MsgFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;SFlag&gt; for SFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;Mode&gt; for Mode","synthetic":false,"types":[]},{"text":"impl Extend&lt;FsFlags&gt; for FsFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;InputFlags&gt; for InputFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;OutputFlags&gt; for OutputFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;ControlFlags&gt; for ControlFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;LocalFlags&gt; for LocalFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;WaitPidFlag&gt; for WaitPidFlag","synthetic":false,"types":[]},{"text":"impl Extend&lt;AddWatchFlags&gt; for AddWatchFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;InitFlags&gt; for InitFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;TimerFlags&gt; for TimerFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;TimerSetTimeFlags&gt; for TimerSetTimeFlags","synthetic":false,"types":[]},{"text":"impl Extend&lt;AccessFlags&gt; for AccessFlags","synthetic":false,"types":[]}];
implementors["onig"] = [{"text":"impl Extend&lt;RegexOptions&gt; for RegexOptions","synthetic":false,"types":[]},{"text":"impl Extend&lt;SearchOptions&gt; for SearchOptions","synthetic":false,"types":[]},{"text":"impl Extend&lt;SyntaxOperator&gt; for SyntaxOperator","synthetic":false,"types":[]},{"text":"impl Extend&lt;SyntaxBehavior&gt; for SyntaxBehavior","synthetic":false,"types":[]},{"text":"impl Extend&lt;TraverseCallbackAt&gt; for TraverseCallbackAt","synthetic":false,"types":[]},{"text":"impl Extend&lt;MetaCharType&gt; for MetaCharType","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl Extend&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl Extend&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; Extend&lt;&lt;A as Array&gt;::Item&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["spinoso_array"] = [{"text":"impl&lt;T&gt; Extend&lt;T&gt; for SmallArray&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Extend&lt;&amp;'a T&gt; for SmallArray&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Copy,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T&gt; Extend&lt;T&gt; for Array&lt;T&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, T&gt; Extend&lt;&amp;'a T&gt; for Array&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: 'a + Copy,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; Extend&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; Extend&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]},{"text":"impl Extend&lt;Error&gt; for Error","synthetic":false,"types":[]}];
implementors["vec_map"] = [{"text":"impl&lt;V&gt; Extend&lt;(usize, V)&gt; for VecMap&lt;V&gt;","synthetic":false,"types":[]},{"text":"impl&lt;'a, V:&nbsp;Copy&gt; Extend&lt;(usize, &amp;'a V)&gt; for VecMap&lt;V&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()