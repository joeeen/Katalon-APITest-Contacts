package helper

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import groovy.json.JsonSlurper
import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import internal.GlobalVariable

public class auth {
	@Keyword(keywordObject = "authentication")
	public String register() {
		String randomEmail = (new helper.random()).email()
		String randomPassword = (new helper.random()).password(10)
		def result = WS.sendRequestAndVerify(findTestObject('Users/Add user', [('email') : randomEmail, ('password') : randomPassword]))
		def slurper = new groovy.json.JsonSlurper()
		def response = slurper.parseText(result.getResponseBodyContent())
		GlobalVariable.token = response.token
		GlobalVariable.email = randomEmail
		GlobalVariable.password = randomPassword
	}

	@Keyword(keywordObject = "authentication")
	public String login(String email, String password) {
		def result = WS.sendRequestAndVerify(findTestObject('Users/Log in user', [('email') : email, ('password') : password]))
		def slurper = new groovy.json.JsonSlurper()
		def response = slurper.parseText(result.getResponseBodyContent())
		GlobalVariable.token = response.token
	}
}