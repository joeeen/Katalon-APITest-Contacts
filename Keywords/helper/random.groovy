package helper

import com.kms.katalon.core.annotation.Keyword
import org.apache.commons.lang.RandomStringUtils as RandomStringUtils


public class random {
	@Keyword(keywordObject = "random generator")
	public String email() {
		String prefix = "user"
		String domain = "sample.com"
		String randomString = RandomStringUtils.randomAlphanumeric(7)
		return "${prefix}${randomString}@${domain}"
	};

	@Keyword(keywordObject = "random generator")
	public String password(int length) {
		return RandomStringUtils.randomAlphanumeric(length)
	}
}
