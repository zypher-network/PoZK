(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[3994],{77673:function(e,t,n){Promise.resolve().then(n.bind(n,30021))},62737:function(e){var t;t=function(){"use strict";var e="millisecond",t="second",n="minute",a="hour",i="week",s="month",r="quarter",u="year",p="date",l="Invalid Date",d=/^(\d{4})[-/]?(\d{1,2})?[-/]?(\d{0,2})[Tt\s]*(\d{1,2})?:?(\d{1,2})?:?(\d{1,2})?[.:]?(\d+)?$/,o=/\[([^\]]+)]|Y{1,4}|M{1,4}|D{1,2}|d{1,4}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS/g,y=function(e,t,n){var a=String(e);return!a||a.length>=t?e:""+Array(t+1-a.length).join(n)+e},m="en",c={};c[m]={name:"en",weekdays:"Sunday_Monday_Tuesday_Wednesday_Thursday_Friday_Saturday".split("_"),months:"January_February_March_April_May_June_July_August_September_October_November_December".split("_"),ordinal:function(e){var t=["th","st","nd","rd"],n=e%100;return"["+e+(t[(n-20)%10]||t[n]||"th")+"]"}};var f="$isDayjsObject",h=function(e){return e instanceof b||!(!e||!e[f])},x=function e(t,n,a){var i;if(!t)return m;if("string"==typeof t){var s=t.toLowerCase();c[s]&&(i=s),n&&(c[s]=n,i=s);var r=t.split("-");if(!i&&r.length>1)return e(r[0])}else{var u=t.name;c[u]=t,i=u}return!a&&i&&(m=i),i||!a&&m},v=function(e,t){if(h(e))return e.clone();var n="object"==typeof t?t:{};return n.date=e,n.args=arguments,new b(n)},T={s:y,z:function(e){var t=-e.utcOffset(),n=Math.abs(t);return(t<=0?"+":"-")+y(Math.floor(n/60),2,"0")+":"+y(n%60,2,"0")},m:function e(t,n){if(t.date()<n.date())return-e(n,t);var a=12*(n.year()-t.year())+(n.month()-t.month()),i=t.clone().add(a,s),r=n-i<0,u=t.clone().add(a+(r?-1:1),s);return+(-(a+(n-i)/(r?i-u:u-i))||0)},a:function(e){return e<0?Math.ceil(e)||0:Math.floor(e)},p:function(l){return({M:s,y:u,w:i,d:"day",D:p,h:a,m:n,s:t,ms:e,Q:r})[l]||String(l||"").toLowerCase().replace(/s$/,"")},u:function(e){return void 0===e}};T.l=x,T.i=h,T.w=function(e,t){return v(e,{locale:t.$L,utc:t.$u,x:t.$x,$offset:t.$offset})};var b=function(){function y(e){this.$L=x(e.locale,null,!0),this.parse(e),this.$x=this.$x||e.x||{},this[f]=!0}var m=y.prototype;return m.parse=function(e){this.$d=function(e){var t=e.date,n=e.utc;if(null===t)return new Date(NaN);if(T.u(t))return new Date;if(t instanceof Date)return new Date(t);if("string"==typeof t&&!/Z$/i.test(t)){var a=t.match(d);if(a){var i=a[2]-1||0,s=(a[7]||"0").substring(0,3);return n?new Date(Date.UTC(a[1],i,a[3]||1,a[4]||0,a[5]||0,a[6]||0,s)):new Date(a[1],i,a[3]||1,a[4]||0,a[5]||0,a[6]||0,s)}}return new Date(t)}(e),this.init()},m.init=function(){var e=this.$d;this.$y=e.getFullYear(),this.$M=e.getMonth(),this.$D=e.getDate(),this.$W=e.getDay(),this.$H=e.getHours(),this.$m=e.getMinutes(),this.$s=e.getSeconds(),this.$ms=e.getMilliseconds()},m.$utils=function(){return T},m.isValid=function(){return this.$d.toString()!==l},m.isSame=function(e,t){var n=v(e);return this.startOf(t)<=n&&n<=this.endOf(t)},m.isAfter=function(e,t){return v(e)<this.startOf(t)},m.isBefore=function(e,t){return this.endOf(t)<v(e)},m.$g=function(e,t,n){return T.u(e)?this[t]:this.set(n,e)},m.unix=function(){return Math.floor(this.valueOf()/1e3)},m.valueOf=function(){return this.$d.getTime()},m.startOf=function(e,r){var l=this,d=!!T.u(r)||r,o=T.p(e),y=function(e,t){var n=T.w(l.$u?Date.UTC(l.$y,t,e):new Date(l.$y,t,e),l);return d?n:n.endOf("day")},m=function(e,t){return T.w(l.toDate()[e].apply(l.toDate("s"),(d?[0,0,0,0]:[23,59,59,999]).slice(t)),l)},c=this.$W,f=this.$M,h=this.$D,x="set"+(this.$u?"UTC":"");switch(o){case u:return d?y(1,0):y(31,11);case s:return d?y(1,f):y(0,f+1);case i:var v=this.$locale().weekStart||0,b=(c<v?c+7:c)-v;return y(d?h-b:h+(6-b),f);case"day":case p:return m(x+"Hours",0);case a:return m(x+"Minutes",1);case n:return m(x+"Seconds",2);case t:return m(x+"Milliseconds",3);default:return this.clone()}},m.endOf=function(e){return this.startOf(e,!1)},m.$set=function(i,r){var l,d=T.p(i),o="set"+(this.$u?"UTC":""),y=((l={}).day=o+"Date",l[p]=o+"Date",l[s]=o+"Month",l[u]=o+"FullYear",l[a]=o+"Hours",l[n]=o+"Minutes",l[t]=o+"Seconds",l[e]=o+"Milliseconds",l)[d],m="day"===d?this.$D+(r-this.$W):r;if(d===s||d===u){var c=this.clone().set(p,1);c.$d[y](m),c.init(),this.$d=c.set(p,Math.min(this.$D,c.daysInMonth())).$d}else y&&this.$d[y](m);return this.init(),this},m.set=function(e,t){return this.clone().$set(e,t)},m.get=function(e){return this[T.p(e)]()},m.add=function(e,r){var p,l=this;e=Number(e);var d=T.p(r),o=function(t){var n=v(l);return T.w(n.date(n.date()+Math.round(t*e)),l)};if(d===s)return this.set(s,this.$M+e);if(d===u)return this.set(u,this.$y+e);if("day"===d)return o(1);if(d===i)return o(7);var y=((p={})[n]=6e4,p[a]=36e5,p[t]=1e3,p)[d]||1,m=this.$d.getTime()+e*y;return T.w(m,this)},m.subtract=function(e,t){return this.add(-1*e,t)},m.format=function(e){var t=this,n=this.$locale();if(!this.isValid())return n.invalidDate||l;var a=e||"YYYY-MM-DDTHH:mm:ssZ",i=T.z(this),s=this.$H,r=this.$m,u=this.$M,p=n.weekdays,d=n.months,y=n.meridiem,m=function(e,n,i,s){return e&&(e[n]||e(t,a))||i[n].slice(0,s)},c=function(e){return T.s(s%12||12,e,"0")},f=y||function(e,t,n){var a=e<12?"AM":"PM";return n?a.toLowerCase():a};return a.replace(o,function(e,a){return a||function(e){switch(e){case"YY":return String(t.$y).slice(-2);case"YYYY":return T.s(t.$y,4,"0");case"M":return u+1;case"MM":return T.s(u+1,2,"0");case"MMM":return m(n.monthsShort,u,d,3);case"MMMM":return m(d,u);case"D":return t.$D;case"DD":return T.s(t.$D,2,"0");case"d":return String(t.$W);case"dd":return m(n.weekdaysMin,t.$W,p,2);case"ddd":return m(n.weekdaysShort,t.$W,p,3);case"dddd":return p[t.$W];case"H":return String(s);case"HH":return T.s(s,2,"0");case"h":return c(1);case"hh":return c(2);case"a":return f(s,r,!0);case"A":return f(s,r,!1);case"m":return String(r);case"mm":return T.s(r,2,"0");case"s":return String(t.$s);case"ss":return T.s(t.$s,2,"0");case"SSS":return T.s(t.$ms,3,"0");case"Z":return i}return null}(e)||i.replace(":","")})},m.utcOffset=function(){return-(15*Math.round(this.$d.getTimezoneOffset()/15))},m.diff=function(e,p,l){var d,o=this,y=T.p(p),m=v(e),c=(m.utcOffset()-this.utcOffset())*6e4,f=this-m,h=function(){return T.m(o,m)};switch(y){case u:d=h()/12;break;case s:d=h();break;case r:d=h()/3;break;case i:d=(f-c)/6048e5;break;case"day":d=(f-c)/864e5;break;case a:d=f/36e5;break;case n:d=f/6e4;break;case t:d=f/1e3;break;default:d=f}return l?d:T.a(d)},m.daysInMonth=function(){return this.endOf(s).$D},m.$locale=function(){return c[this.$L]},m.locale=function(e,t){if(!e)return this.$L;var n=this.clone(),a=x(e,t,!0);return a&&(n.$L=a),n},m.clone=function(){return T.w(this.$d,this)},m.toDate=function(){return new Date(this.valueOf())},m.toJSON=function(){return this.isValid()?this.toISOString():null},m.toISOString=function(){return this.$d.toISOString()},m.toString=function(){return this.$d.toUTCString()},y}(),w=b.prototype;return v.prototype=w,[["$ms",e],["$s",t],["$m",n],["$H",a],["$W","day"],["$M",s],["$y",u],["$D",p]].forEach(function(e){w[e[1]]=function(t){return this.$g(t,e[0],e[1])}}),v.extend=function(e,t){return e.$i||(e(t,b,v),e.$i=!0),v},v.locale=x,v.isDayjs=h,v.unix=function(e){return v(1e3*e)},v.en=c[m],v.Ls=c,v.p={},v},e.exports=t()},30021:function(e,t,n){"use strict";n.r(t),n.d(t,{default:function(){return q}});var a,i,s,r,u,p,l,d,o=n(57437),y=n(2265),m=n(56800),c=n.n(m),f=n(48185),h=n(4410);function x(){return(x=Object.assign?Object.assign.bind():function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var a in n)({}).hasOwnProperty.call(n,a)&&(e[a]=n[a])}return e}).apply(null,arguments)}var v=function(e){return h.createElement("svg",x({xmlns:"http://www.w3.org/2000/svg",width:72,height:72,fill:"none",viewBox:"0 0 72 72"},e),h.createElement("mask",{id:"a",width:72,height:72,x:0,y:0,maskUnits:"userSpaceOnUse",style:{maskType:"alpha"}},i||(i=h.createElement("path",{fill:"#D9D9D9",d:"M0 0h72v72H0z"}))),s||(s=h.createElement("g",{mask:"url(#a)"},h.createElement("path",{stroke:"#fff",strokeWidth:2,d:"M53 28V16.766c0-3.929-3.713-6.797-7.514-5.806l-34 8.87A6 6 0 0 0 7 25.636V57"}),h.createElement("path",{fill:"#fff",fillRule:"evenodd",d:"M12 23a6 6 0 0 0-6 6v28a6 6 0 0 0 6 6h45a6 6 0 0 0 6-6v-5H51a9 9 0 1 1 0-18h12v-5a6 6 0 0 0-6-6z",clipRule:"evenodd"}),h.createElement("path",{fill:"#fff",fillRule:"evenodd",d:"M51 37a6 6 0 0 0 0 12h14.37c.9 0 1.63-.73 1.63-1.63v-8.74c0-.9-.73-1.63-1.63-1.63zm1 9a3 3 0 1 0 0-6 3 3 0 0 0 0 6",clipRule:"evenodd"}))))};function T(){return(T=Object.assign?Object.assign.bind():function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var a in n)({}).hasOwnProperty.call(n,a)&&(e[a]=n[a])}return e}).apply(null,arguments)}var b=function(e){return h.createElement("svg",T({xmlns:"http://www.w3.org/2000/svg",width:72,height:72,fill:"none",viewBox:"0 0 72 72"},e),h.createElement("mask",{id:"a",width:72,height:72,x:0,y:0,maskUnits:"userSpaceOnUse",style:{maskType:"alpha"}},r||(r=h.createElement("path",{fill:"#D9D9D9",d:"M0 0h72v72H0z"}))),u||(u=h.createElement("g",{mask:"url(#a)"},h.createElement("rect",{width:12,height:26,x:10,y:42,fill:"#fff",rx:3}),h.createElement("rect",{width:12,height:34,x:30,y:34,fill:"#fff",rx:3}),h.createElement("rect",{width:12,height:40,x:50,y:28,fill:"#fff",rx:3}),h.createElement("path",{stroke:"#fff",strokeLinecap:"round",strokeWidth:3,d:"m11.752 33.76 47.496-23.52"}),h.createElement("path",{stroke:"#fff",strokeLinecap:"round",strokeLinejoin:"round",strokeWidth:3,d:"m51.208 7.526 8.04 2.714-2.715 8.04"}))))};function w(){return(w=Object.assign?Object.assign.bind():function(e){for(var t=1;t<arguments.length;t++){var n=arguments[t];for(var a in n)({}).hasOwnProperty.call(n,a)&&(e[a]=n[a])}return e}).apply(null,arguments)}var g=function(e){return h.createElement("svg",w({xmlns:"http://www.w3.org/2000/svg",width:72,height:72,fill:"none",viewBox:"0 0 72 72"},e),h.createElement("mask",{id:"a",width:72,height:72,x:0,y:0,maskUnits:"userSpaceOnUse",style:{maskType:"alpha"}},p||(p=h.createElement("path",{fill:"#D9D9D9",d:"M0 0h72v72H0z"}))),l||(l=h.createElement("g",{mask:"url(#a)"},h.createElement("path",{fill:"#fff",d:"M25.621 23.515A2 2 0 0 1 27.561 22h16.877a2 2 0 0 1 1.94 1.515L51 42H21zM8.621 46.515A2 2 0 0 1 10.561 45h16.877a2 2 0 0 1 1.94 1.515L34 65H4zM42.621 46.515A2 2 0 0 1 44.561 45h16.877a2 2 0 0 1 1.94 1.515L68 65H38z"}),h.createElement("path",{stroke:"#fff",strokeLinecap:"round",strokeWidth:3,d:"M36 8v6M16.201 16.201l4.243 4.243M55.799 16.201l-4.243 4.243M64 36h-6M14 36H8"}))))},M=n(25566),k=n(49354),S=n(87212),D=n(49999),$=n(68361),j=n(27120),N=()=>(0,o.jsx)("div",{className:"flex items-center justify-center animate-spin",children:(0,o.jsx)(j.Z,{className:"scale-y-[-1]",height:"40px",width:"40px"})}),O=n(75897),C=n(43710),A=n(43665),_=n(35280);let E=e=>{let{item:t,pending:n,onClaim:a}=e,{title:i,value:s,Icon:r,className:u,borderLeftClassName:p}=t;return(0,o.jsxs)(f.Zb,{className:(0,k.cn)("basis-1/3 relative min-h-[184px] pl-[50px]",u),children:[(0,o.jsx)("p",{className:"font-light text-[20px] text-nowrap  pt-[10px]",children:i}),(0,o.jsx)("h3",{className:"text-[44px] font-semibold text-nowrap pt-[10px]",children:n?(0,o.jsx)(N,{}):s}),(0,o.jsx)("div",{className:c()("mt-3 flex justify-end",{"hidden pointer-events-none":!["Collected Rewards"].includes(i)||"0"===s}),children:(0,o.jsx)(O.z,{onClick:()=>null==a?void 0:a(),children:"Claim Rewards"})}),(0,o.jsx)(r,{className:"absolute top-[56px]  right-[10%] size-[72px]"}),(0,o.jsx)("div",{className:(0,k.cn)("w-[8px] h-[88px] absolute top-[48px] left-0 z-10 rounded-r-[8px]",p)})]})};var F=(0,y.memo)(()=>{let{address:e}=(0,S.m)(),{data:t,pending:n}=(0,$.Z)(e=>e.reward),a=(0,$.Z)(e=>e.claimedAmount),i=(0,$.Z)(e=>e.reset),[s,r]=(0,y.useState)("0"),u=(0,y.useMemo)(()=>{var e,n,i,r;let u=new D.ZP("0"),p=new D.ZP("0");for(let a of null!==(e=null==t?void 0:t.claimList)&&void 0!==e?e:[])u=u.plus(null!==(n=a.claim)&&void 0!==n?n:"0"),p=p.plus(null!==(r=null!==(i=a.claim)&&void 0!==i?i:a.estimate)&&void 0!==r?r:"0");return[{title:"Total Rewards Estimate",value:p.div(D.ze).toFormat(),Icon:v,borderLeftClassName:"bg-[#FACC16]",className:"bg-gradient-to-b from-[#9277FD] to-[#674EFF]"},{title:"Collected Rewards",value:s,Icon:b,borderLeftClassName:"bg-[#BF38FF]",className:"bg-gradient-to-b from-[#C8D254] to-[#71A61A]"},{title:"Claimed Rewards",value:a.data,Icon:g,borderLeftClassName:"bg-[#3B5AFF]",className:"bg-gradient-to-b from-[#E7C56D] to-[#E18802]"}]},[e,t,s,a]),p=async()=>{r("0");try{let t=new C.Z(A.iF[A.lG].Stake,_),n=await t.readContractData("claimable",[e]);r(new D.ZP(n.toString()).div(D.ze).toFormat())}catch(e){r("0")}},l=async()=>{try{let t=new C.Z(A.iF[A.lG].Stake,_);await t.writeContractMethod("claim",[e]),i("reward")}catch(e){console.log(e)}};return(0,y.useEffect)(()=>{t&&e&&p()},[t,e]),(0,o.jsx)("div",{className:"flex justify-between items-stretch gap-[24px]",children:u.map(e=>(0,o.jsx)(E,{item:e,pending:n,onClaim:l},M.title))})}),P=n(62737),z=n.n(P),L=n(47304),Z=n(41942);let U=(0,n(39099).Ue)(e=>({selected:null,fetching:!1,setSelectEpoch:t=>{e({selected:t})}}));var I=n(30926),H=n(54662),Y=n(72039),R=JSON.parse('[{"inputs":[],"name":"InvalidInitialization","type":"error"},{"inputs":[],"name":"NotInitializing","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"OwnableInvalidOwner","type":"error"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"OwnableUnauthorizedAccount","type":"error"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"int256","name":"alphaNumerator","type":"int256"},{"indexed":false,"internalType":"int256","name":"alphaDenominator","type":"int256"}],"name":"Alpha","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"int256","name":"betaNumerator","type":"int256"},{"indexed":false,"internalType":"int256","name":"betaDenominator","type":"int256"}],"name":"Beta","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"int256","name":"gammaNumerator","type":"int256"},{"indexed":false,"internalType":"int256","name":"gammaDenominator","type":"int256"}],"name":"Gamma","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint64","name":"version","type":"uint64"}],"name":"Initialized","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"address","name":"miner","type":"address"},{"indexed":false,"internalType":"uint256","name":"amount","type":"uint256"}],"name":"MinerCollect","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"address","name":"miner","type":"address"},{"indexed":false,"internalType":"uint256","name":"work","type":"uint256"}],"name":"MinerLabor","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"minerMaxPer","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"minerMinPer","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"playerMaxNum","type":"uint256"}],"name":"MinerPlayerPer","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"address","name":"player","type":"address"},{"indexed":false,"internalType":"uint256","name":"amount","type":"uint256"}],"name":"PlayerCollect","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"address","name":"player","type":"address"},{"indexed":false,"internalType":"uint256","name":"play","type":"uint256"}],"name":"PlayerLabor","type":"event"},{"inputs":[],"name":"alphaDenominator","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"alphaNumerator","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"betaDenominator","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"betaNumerator","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"gammaDenominator","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"gammaNumerator","outputs":[{"internalType":"int256","name":"","type":"int256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"_addresses","type":"address"},{"internalType":"int256","name":"_alphaNumerator","type":"int256"},{"internalType":"int256","name":"_alphaDenominator","type":"int256"},{"internalType":"int256","name":"_betaNumerator","type":"int256"},{"internalType":"int256","name":"_betaDenominator","type":"int256"},{"internalType":"int256","name":"_gammaNumerator","type":"int256"},{"internalType":"int256","name":"_gammaDenominator","type":"int256"},{"internalType":"uint256","name":"_minerMaxPer","type":"uint256"},{"internalType":"uint256","name":"_minerMinPer","type":"uint256"},{"internalType":"uint256","name":"_playerMaxNum","type":"uint256"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"epoch","type":"uint256"},{"internalType":"address","name":"miner","type":"address"}],"name":"minerBatchCollect","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"epoch","type":"uint256"},{"internalType":"address","name":"prover","type":"address"},{"internalType":"address","name":"miner","type":"address"}],"name":"minerCollect","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"minerMaxPer","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"minerMinPer","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"epoch","type":"uint256"},{"internalType":"address","name":"player","type":"address"}],"name":"playerBatchCollect","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"epoch","type":"uint256"},{"internalType":"address","name":"prover","type":"address"},{"internalType":"address","name":"player","type":"address"}],"name":"playerCollect","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"playerMaxNum","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"_addresses","type":"address"}],"name":"setAddresses","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"int256","name":"_alphaNumerator","type":"int256"},{"internalType":"int256","name":"_alphaDenominator","type":"int256"}],"name":"setAlpha","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"int256","name":"_betaNumerator","type":"int256"},{"internalType":"int256","name":"_betaDenominator","type":"int256"}],"name":"setBeta","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"int256","name":"_gammaNumerator","type":"int256"},{"internalType":"int256","name":"_gammaDenominator","type":"int256"}],"name":"setGamma","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"_minerMaxPer","type":"uint256"},{"internalType":"uint256","name":"_minerMinPer","type":"uint256"},{"internalType":"uint256","name":"_playerMaxNum","type":"uint256"}],"name":"setMinerPlayerPer","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"address","name":"player","type":"address"},{"internalType":"address","name":"miner","type":"address"}],"name":"work","outputs":[],"stateMutability":"nonpayable","type":"function"}]');let B=()=>{let e=A.iF[A.lG].Reward;return new C.Z(e,R)};var W=(0,y.memo)(()=>{var e;let[t,n]=(0,y.useState)(!1),a=(0,$.Z)(e=>e.reset),{epoch:i,setEpoch:s}=U((0,Y.N)(e=>({epoch:e.selected,setEpoch:e.setSelectEpoch}))),{address:r}=(0,S.m)(),u=(0,y.useCallback)(async()=>{if(i){n(!0);try{let e=B();await e.writeContractMethod("minerBatchCollect",[BigInt(i.id),r]),a("reward"),a("epoches"),s(null)}catch(e){console.log(e)}finally{n(!1)}}},[i,r]),p=(0,y.useCallback)(()=>{s(null)},[]);return(0,o.jsx)(H.Vq,{open:!!i,onOpenChange:p,children:(0,o.jsxs)(H.cZ,{className:"w-[512px] fixed",children:[(0,o.jsx)(H.fK,{children:(0,o.jsx)(H.$N,{children:"Epoch ".concat(null==i?void 0:i.epoch)})}),(0,o.jsxs)("div",{className:"flex flex-col justify-center items-center mt-[60px] z-10",children:[(0,o.jsx)("p",{className:"text-[20px] font-light",children:"Estimated Reward Points"}),(0,o.jsx)("h3",{className:"text-[48px] font-medium",children:new D.ZP(null!==(e=null==i?void 0:i.estimate)&&void 0!==e?e:"0").div(D.ze).toFormat()}),(0,o.jsx)(O.z,{type:"submit",variant:"default",className:"h-[62px] w-[212px]  mt-[60px] mb-[20px]  rounded-[100px]  font-light text-[20px] ",isLoading:t,disabled:t,onClick:u,children:"Collect"})]}),(0,o.jsx)("img",{className:"absolute top-[40px] left-0 w-full",src:"/rewards/claim_bg.png",alt:"dashboard",width:512,height:248})]})})}),V=n(99441);(a=d||(d={}))["Epoch/Phase"]="Epoch/Phase",a["Start/End Date"]="Start/End Date",a["Total Uptime"]="Total Uptime",a["Estimated Reward"]="Estimated Reward",a.Active="Active";let G=["Epoch/Phase","Start/End Date","Total Uptime","Estimated Reward","Active"];(0,V.cn)({key:"EpochPhaseData",default:[]}),(0,V.cn)({key:"ChooseIndex",default:void 0});var J=(0,y.memo)(()=>{let[e,t]=(0,y.useState)(!1),n=U(e=>e.setSelectEpoch),{epoches:a,reward:i}=(0,$.Z)((0,Y.N)(e=>({reward:e.reward,epoches:e.epoches}))),s=(0,y.useMemo)(()=>{if(!i.pending){let s=[];for(let r of a.data){var e,t,n;let{id:a}=r,u="0",p=!1,l=[];for(let s of null!==(t=null===(e=i.data)||void 0===e?void 0:e.claimList)&&void 0!==t?t:[])s.epoch!==a||(u=new D.ZP(null!==(n=s.claim)&&void 0!==n?n:s.estimate).plus(u).toString(10),l.push(s.prover),s.claim||(p=!0));s.push({...r,estimate:u,claimable:p,provers:l,epoch:a})}return s}return[]},[a,i]);return(0,o.jsxs)(f.Zb,{children:[(0,o.jsxs)(f.Ol,{className:"flex flex-row justify-between items-center pb-[24px]",children:[(0,o.jsx)(f.ll,{children:"Epoch/phase"}),(0,o.jsxs)(f.Zb,{onClick:()=>t(e=>!e),className:"bg-primary px-[16px] py-[0] gap-[4px] text-[16px] font-light flex flex-row justify-end items-center h-[36px] mt-0 cursor-pointer hover:bg-primary/90",children:[(0,o.jsx)(Z.vpT,{}),"View All"]})]}),(0,o.jsxs)(L.iA,{children:[(0,o.jsx)(L.xD,{children:(0,o.jsx)(L.SC,{className:"border-[#1F2D4E]",children:G.map((e,t)=>(0,o.jsx)(L.ss,{className:"".concat([0,1,2].includes(t)?"text-left":"text-right"),children:e},e))})}),(0,o.jsxs)(L.RM,{children:[(0,o.jsx)(L.SC,{children:(0,o.jsx)(L.pj,{colSpan:G.length,className:"h-[16px] bg-transparent"})}),(e?s:s.slice(0,4)).map(e=>(0,o.jsxs)(L.SC,{children:[(0,o.jsx)(L.pj,{children:(0,o.jsxs)("div",{className:"flex flex-row items-center gap-[40px]",children:["Epoch ".concat(e.id),e.endAt?null:(0,o.jsx)("div",{className:"h-[36px] px-[16px] bg-[#82c01e] rounded-[8px] text-[#0A1223]  font-light text-[18px] leading-[36px] ",children:"Current"})]})}),(0,o.jsx)(L.pj,{className:"justify-start text-[#FFFFFF]",children:(0,o.jsxs)("div",{className:"flex flex-col gap-1",children:[(0,o.jsxs)("div",{children:[z()(1e3*+e.startAt).format("MMM DD.YYYY hh:mm:ss A")," -"]}),e.endAt&&(0,o.jsx)("div",{children:z()(1e3*+e.endAt).format("MMM DD.YYYY hh:mm:ss A")})]})}),(0,o.jsx)(L.pj,{className:"justify-start text-[#FFFFFF]",children:e.endAt?(0,I.L)(new Date(1e3*+e.startAt),new Date(1e3*+e.endAt)):"--"}),(0,o.jsx)(L.pj,{className:"justify-end text-[#FACC16]",children:new D.ZP(e.estimate).div(D.ze).toFormat()}),(0,o.jsx)(L.pj,{className:"justify-end",children:e.endAt&&"0"!==e.estimate?e.claimable?(0,o.jsx)(O.z,{variant:"outline",onClick:()=>n(e),children:"Collect"}):(0,o.jsx)(O.z,{variant:"ghost",disabled:!0,children:"Collected"}):(0,o.jsx)(O.z,{variant:"outline",className:"opacity-0 pointer-events-none",children:"Collected"})})]},e.id))]})]}),(0,o.jsx)(W,{})]})});function q(){return(0,o.jsxs)("div",{className:"flex flex-col gap-[24px] pb-[54px]",children:[(0,o.jsx)(F,{}),(0,o.jsx)(J,{})]})}},68361:function(e,t,n){"use strict";let a=(0,n(39099).Ue)(e=>({epoches:{data:[],pending:!1},provers:{data:[],pending:!1},reward:{data:null,pending:!1},staking:{data:[],pending:!1},claimedAmount:{data:"0",pending:!1},setData(t,n){e({[t]:{data:n,pending:!1}})},reset(t){e({[t]:{data:"reward"===t?null:[],pending:!0}})}}));t.Z=a},49999:function(e,t,n){"use strict";n.d(t,{ze:function(){return i}});var a=n(19449);let i=new a.Z("10").exponentiatedBy(18);a.O.config({DECIMAL_PLACES:2,ROUNDING_MODE:a.O.ROUND_HALF_DOWN}),t.ZP=a.Z},30926:function(e,t,n){"use strict";n.d(t,{L:function(){return s},x:function(){return r}});var a=n(62737),i=n.n(a);let s=(e,t)=>{let[n,a,s]=i().duration((i()(t).unix()-i()(e).unix())*1e3).format("D_HH_mm").split("_");return"".concat(n,"Day - ").concat(a,"Hr - ").concat(s,"min")},r=e=>i()(e.endAt?1e3*+e.endAt:void 0).isToday()},56800:function(e,t){var n;/*!
	Copyright (c) 2018 Jed Watson.
	Licensed under the MIT License (MIT), see
	http://jedwatson.github.io/classnames
*/!function(){"use strict";var a={}.hasOwnProperty;function i(){for(var e="",t=0;t<arguments.length;t++){var n=arguments[t];n&&(e=s(e,function(e){if("string"==typeof e||"number"==typeof e)return e;if("object"!=typeof e)return"";if(Array.isArray(e))return i.apply(null,e);if(e.toString!==Object.prototype.toString&&!e.toString.toString().includes("[native code]"))return e.toString();var t="";for(var n in e)a.call(e,n)&&e[n]&&(t=s(t,n));return t}(n)))}return e}function s(e,t){return t?e?e+" "+t:e+t:e}e.exports?(i.default=i,e.exports=i):void 0!==(n=(function(){return i}).apply(t,[]))&&(e.exports=n)}()},35280:function(e){"use strict";e.exports=JSON.parse('[{"inputs":[{"internalType":"address","name":"target","type":"address"}],"name":"AddressEmptyCode","type":"error"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"AddressInsufficientBalance","type":"error"},{"inputs":[],"name":"FailedInnerCall","type":"error"},{"inputs":[],"name":"InvalidInitialization","type":"error"},{"inputs":[],"name":"NotInitializing","type":"error"},{"inputs":[{"internalType":"address","name":"owner","type":"address"}],"name":"OwnableInvalidOwner","type":"error"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"OwnableUnauthorizedAccount","type":"error"},{"inputs":[{"internalType":"address","name":"token","type":"address"}],"name":"SafeERC20FailedOperation","type":"error"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address[]","name":"accounts","type":"address[]"},{"indexed":false,"internalType":"uint256[]","name":"amounts","type":"uint256[]"}],"name":"AddAllowlist","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"uint256","name":"amount","type":"uint256"}],"name":"AddUnstaking","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"uint256","name":"amount","type":"uint256"}],"name":"ClaimUnstaking","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint64","name":"version","type":"uint64"}],"name":"Initialized","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"int256","name":"changed","type":"int256"},{"indexed":false,"internalType":"uint256","name":"staking","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"total","type":"uint256"}],"name":"MinerStakeChange","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"id","type":"uint256"}],"name":"MinerTestCancel","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"id","type":"uint256"},{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"uint256","name":"overtime","type":"uint256"},{"indexed":false,"internalType":"bytes","name":"inputs","type":"bytes"},{"indexed":false,"internalType":"bytes","name":"publics","type":"bytes"}],"name":"MinerTestCreate","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"id","type":"uint256"},{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"address","name":"prover","type":"address"}],"name":"MinerTestRequire","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"id","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"submitAt","type":"uint256"}],"name":"MinerTestSubmit","type":"event"},{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"int256","name":"changed","type":"int256"},{"indexed":false,"internalType":"uint256","name":"staking","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"total","type":"uint256"}],"name":"PlayerStakeChange","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"uint256","name":"epoch","type":"uint256"},{"indexed":false,"internalType":"address","name":"prover","type":"address"},{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"int256","name":"changed","type":"int256"},{"indexed":false,"internalType":"uint256","name":"staking","type":"uint256"},{"indexed":false,"internalType":"uint256","name":"total","type":"uint256"}],"name":"ProverStakeChange","type":"event"},{"anonymous":false,"inputs":[{"indexed":false,"internalType":"address","name":"account","type":"address"},{"indexed":false,"internalType":"uint256","name":"amount","type":"uint256"}],"name":"SubAllowlist","type":"event"},{"inputs":[{"internalType":"address[]","name":"accounts","type":"address[]"},{"internalType":"uint256[]","name":"amounts","type":"uint256[]"}],"name":"addAllowlist","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"account","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"addUnstaking","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"","type":"address"}],"name":"allowlist","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"claim","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"claimable","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"_addresses","type":"address"},{"internalType":"uint256","name":"_minStakeAmount","type":"uint256"}],"name":"initialize","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"address","name":"account","type":"address"}],"name":"isMiner","outputs":[{"internalType":"bool","name":"","type":"bool"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"minStakeAmount","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"minerStake","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"miner","type":"address"},{"internalType":"address","name":"prover","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"minerStakeFor","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"address","name":"account","type":"address"}],"name":"minerStaking","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"id","type":"uint256"},{"internalType":"bytes","name":"inputs","type":"bytes"},{"internalType":"bytes","name":"publics","type":"bytes"}],"name":"minerTest","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"id","type":"uint256"}],"name":"minerTestCancel","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"id","type":"uint256"},{"internalType":"bool","name":"autoNew","type":"bool"},{"internalType":"bytes","name":"proof","type":"bytes"}],"name":"minerTestSubmit","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"}],"name":"minerTotalStaking","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"from","type":"address"},{"internalType":"address","name":"to","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"minerTransferStaking","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"minerUnstake","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"playerStake","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"player","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"playerStakeFor","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"account","type":"address"}],"name":"playerStaking","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"playerTotalStaking","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"playerUnstake","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"proverStake","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"address","name":"account","type":"address"}],"name":"proverStaking","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"}],"name":"proverTotalStaking","outputs":[{"internalType":"uint256","name":"","type":"uint256"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"address","name":"prover","type":"address"},{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"proverUnstake","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"_addresses","type":"address"}],"name":"setAddresses","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"uint256","name":"_minStakeAmount","type":"uint256"}],"name":"setMinStakeAmount","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"}]')}},function(e){e.O(0,[6900,691,7699,2183,3494,8610,2652,4453,7610,9174,3304,8918,2971,7023,1744],function(){return e(e.s=77673)}),_N_E=e.O()}]);