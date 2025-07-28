document.addEventListener('DOMContentLoaded', function() {
    // 桌面版搜索框清除按钮功能
    const searchInputDesktop = document.getElementById('search-input-desktop');
    const clearButtonDesktop = document.getElementById('search-clear-desktop');

    if (searchInputDesktop && clearButtonDesktop) {
        // 初始检查输入框是否有内容
        if (searchInputDesktop.value.length > 0) {
            clearButtonDesktop.style.display = 'inline-block';
        }
        
        // 监听输入事件，显示或隐藏清除按钮
        searchInputDesktop.addEventListener('input', function() {
            if (searchInputDesktop.value.length > 0) {
                clearButtonDesktop.style.display = 'inline-block';
            } else {
                clearButtonDesktop.style.display = 'none';
            }
        });

        // 点击清除按钮时清空输入框内容
        clearButtonDesktop.addEventListener('click', function() {
            searchInputDesktop.value = '';
            clearButtonDesktop.style.display = 'none';
            searchInputDesktop.focus(); // 保持输入框焦点
        });
    }
    
    // 移动版搜索框清除按钮功能
    const searchInputMobile = document.getElementById('search-input-mobile');
    const clearButtonMobile = document.getElementById('search-clear-mobile');
    
    if (searchInputMobile && clearButtonMobile) {
        // 初始检查输入框是否有内容
        if (searchInputMobile.value.length > 0) {
            clearButtonMobile.style.display = 'inline-block';
        } else {
            clearButtonMobile.style.display = 'none';
        }
        
        // 监听输入事件，显示或隐藏清除按钮
        searchInputMobile.addEventListener('input', function() {
            if (searchInputMobile.value.length > 0) {
                clearButtonMobile.style.display = 'inline-block';
            } else {
                clearButtonMobile.style.display = 'none';
            }
        });
    }
});
