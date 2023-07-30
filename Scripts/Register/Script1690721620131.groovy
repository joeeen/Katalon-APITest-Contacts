import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

String randomEmail = CustomKeywords.'helper.random.email'()

String randomPassword = CustomKeywords.'helper.random.password'(7)

response = WS.sendRequestAndVerify(findTestObject('Users/Add user', [('email') : randomEmail, ('password') : randomPassword]))