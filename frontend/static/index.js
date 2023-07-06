$(function () {
    function waitForElm(selector) {
        return new Promise(resolve => {
            if (document.querySelector(selector)) {
                return resolve(document.querySelector(selector));
            }

            const observer = new MutationObserver(mutations => {
                if (document.querySelector(selector)) {
                    resolve(document.querySelector(selector));
                    observer.disconnect();
                }
            });

            observer.observe(document.body, {
                childList: true,
                subtree: true
            });
        });
    }

    waitForElm('.typeahead').then((elm) => {
        const contacts = new Bloodhound({
            datumTokenizer: Bloodhound.tokenizers.whitespace,
            queryTokenizer: Bloodhound.tokenizers.whitespace,
            prefetch: {
                url: '../data.json',
                cache: false
            }
        });

        $('.typeahead').typeahead({
            highlight: true,
        }, {
            name: 'contacts',
            source: contacts
        }).on('typeahead:selected', function ($e, datum) {
            $('.typeahead').trigger('blur');
        });
    });
});