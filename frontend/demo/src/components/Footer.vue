<template>
  <div>
    <div v-if="!isMobile" class="footer">
      <div class="footer-top">
        <div class="footer-container">
          <!-- 底部左侧 -->
          <div class="footer-top-left">
            <div class="footer-header">
              <img src="../assets/img/logo.png" alt="">
              <span>Game Zone</span>
            </div>
            <div class="footer-desc">
              Ut placesddsdsd sdsdsds sgsrrg asfeg sdgbzv sgshsdsd sdf sdf as alsgjsdds gasdkjgh sjkg askdgsdakjhg ksjg
            </div>
          </div>
          <!-- 底部右侧 -->
          <div class="footer-top-right">
            <dl>
              <dt>About Us</dt>
              <dd>Our team</dd>
              <dd>Company</dd>
              <dd>Jobs</dd>
              <dd>Newsdslster</dd>
            </dl>
  
            <dl>
              <dt>About Us</dt>
              <dd>Our team</dd>
              <dd>Company</dd>
              <dd>Jobs</dd>
              <dd>Newsdslster</dd>
            </dl>
  
            <dl>
              <dt>About Us</dt>
              <dd>Our team</dd>
              <dd>Company</dd>
              <dd>Jobs</dd>
              <dd>Newsdslster</dd>
            </dl>
  
            <dl>
              <dt>About Us</dt>
              <dd>Our team</dd>
              <dd>Company</dd>
              <dd>Jobs</dd>
              <dd>Newsdslster</dd>
            </dl>
          </div>
        </div>
      </div>
      <div class="footer-bottom">
        <div class="footer-container">
          <div class="left">©2021, {{metadata.name}} Inc.</div>
          <div class="right">All Rights Reserved.</div>
        </div>
      </div>
    </div>
    <div v-else class="mobile-footer">
      <div class="icon-with">
        <img src="../assets/img/icon-fb.png" alt="" height="15">
        <img src="../assets/img/icon-t.png" alt="" height="15">
        <img src="../assets/img/icon-gp.png" alt="" height="15">
        <img src="../assets/img/icon-yt.png" alt="" height="15">
        <img src="../assets/img/icon-ig.png" alt="" height="15">
      </div>
      <div class="mobile-footer-bottom">
        <div class="left">©2021, {{metadata.name}} Inc.</div>
        <div class="right">All Rights Reserved.</div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    isMobile: {
      require: true
    },
    metadata: {
      requrie: true
    }
  }
}
</script>

<style scoped>
  .footer {
    color: #f6c94a;
    font-size: 14px;
  }
  .footer-container {
    display: flex;
    justify-content: space-between;
    max-width: 1120px;
    margin: 0 auto;
    /* background-color: red; */
  }
  .footer .footer-top {
    padding: 81px 0;
    height: 362px;
    display: flex;
    justify-content: center;
    background: url('../assets/img/bg-footer-top.png') no-repeat;
    background-size: cover;
  }
  .footer .footer-top .footer-top-left {
    width: 268px;
  }
  .footer .footer-top .footer-top-left .footer-header {
    margin-bottom: 21px;
    font-size: 20px;
    font-weight: 500;
    color: #f8ae1c;
  }
  .footer .footer-top .footer-top-left .footer-header img {
    margin-right: 17px;
  }
  .footer .footer-top .footer-top-left .footer-desc {
    line-height: 25px;
  }
  .footer .footer-top .footer-top-right {
    display: flex;
    justify-content: space-between;
  }
  .footer .footer-top .footer-top-right dl {
    margin-right: 36px;
    padding-top: 36px;
    width: 158px;
  }
  .footer .footer-top .footer-top-right dl:nth-last-of-type(4) {
    background: url('../assets/img/footer-menu-top-line1.png') no-repeat left top;
  }
  .footer .footer-top .footer-top-right dl:nth-last-of-type(3) {
    background: url('../assets/img/footer-menu-top-line2.png') no-repeat left top;
  }
  .footer .footer-top .footer-top-right dl:nth-last-of-type(2) {
    background: url('../assets/img/footer-menu-top-line3.png') no-repeat left top;
  }
  .footer .footer-top .footer-top-right dl:nth-last-of-type(1) {
    background: url('../assets/img/footer-menu-top-line4.png') no-repeat left top;
  }
  .footer .footer-top .footer-top-right dt {
    color: #f8ae1c;
    font-weight: 300;
    margin-bottom: 24px;
  }
  .footer .footer-top .footer-top-right dd {
    font-weight: 200;
    cursor: pointer;
  }
  .footer .footer-bottom {
    padding-left: 40px;
    padding-right: 40px;
    background-color: #000;
    height: 71px;
    color: #f8ae1c;
    font-size: 12px;
  }
  .footer .footer-bottom .footer-container {
    align-items: center;
    height: 100%;
  }
  .mobile-footer {
    background-color: #191919;
    height: 90px;
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    justify-content: center;
    padding: 20px 0;
  }
  .mobile-footer .icon-with {
    display: flex;
    justify-content: space-between;
    flex-wrap: nowrap;
    width: 70%;
    margin-bottom: 20px;
  }
  .mobile-footer .mobile-footer-bottom {
    width: 70%;
    display: flex;
    justify-content: space-between;
    color: #f8ae1c;
    font-size: 12px;
    font-weight: 300;
  }
</style>
